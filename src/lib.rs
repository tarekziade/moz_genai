use std::ffi::CStr;
use std::ffi::CString;
use std::io::Write;
use std::os::raw::c_char;
use std::path::PathBuf;

#[cfg(feature = "accelerate")]
extern crate accelerate_src;

use anyhow::{Error as E, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::t5;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

extern crate kuchiki;
use crate::kuchiki::traits::TendrilSink;

const DTYPE: DType = DType::F32;
const MODEL_NAME: &str = "Falconsai/text_summarization";

struct T5ModelBuilder {
    device: Device,
    config: t5::Config,
    weights_filename: Vec<PathBuf>,
}

impl T5ModelBuilder {
    pub fn load(model_id: String, revision: String) -> Result<(Self, Tokenizer)> {
        let device = Device::Cpu;
        let repo = Repo::with_revision(model_id.clone(), RepoType::Model, revision);
        let api = Api::new().unwrap();
        let api = api.repo(repo);
        let config_filename = api.get("config.json").unwrap();
        let tokenizer_filename = api.get("tokenizer.json").unwrap();
        let weights_filename = if model_id == "google/flan-t5-xxl" {
            vec![
                api.get("model-00001-of-00005.safetensors").unwrap(),
                api.get("model-00002-of-00005.safetensors").unwrap(),
                api.get("model-00003-of-00005.safetensors").unwrap(),
                api.get("model-00004-of-00005.safetensors").unwrap(),
                api.get("model-00005-of-00005.safetensors").unwrap(),
            ]
        } else if model_id == "google/flan-ul2" {
            vec![
                api.get("model-00001-of-00008.safetensors").unwrap(),
                api.get("model-00002-of-00008.safetensors").unwrap(),
                api.get("model-00003-of-00008.safetensors").unwrap(),
                api.get("model-00004-of-00008.safetensors").unwrap(),
                api.get("model-00005-of-00008.safetensors").unwrap(),
                api.get("model-00006-of-00008.safetensors").unwrap(),
                api.get("model-00007-of-00008.safetensors").unwrap(),
                api.get("model-00008-of-00008.safetensors").unwrap(),
            ]
        } else {
            vec![api.get("model.safetensors").unwrap()]
        };
        let config = std::fs::read_to_string(config_filename).unwrap();
        let mut config: t5::Config = serde_json::from_str(&config).unwrap();
        config.use_cache = true;
        let tokenizer = Tokenizer::from_file(tokenizer_filename)
            .map_err(E::msg)
            .unwrap();
        Ok((
            Self {
                device,
                config,
                weights_filename,
            },
            tokenizer,
        ))
    }

    pub fn build_model(&self) -> Result<t5::T5ForConditionalGeneration> {
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&self.weights_filename, DTYPE, &self.device)
                .unwrap()
        };
        Ok(t5::T5ForConditionalGeneration::load(vb, &self.config).unwrap())
    }
}

#[no_mangle]
pub extern "C" fn ping() -> *mut c_char {
    let result_c_string = CString::new("Pong").unwrap();
    result_c_string.into_raw()
}

pub fn trim_whitespace(s: &str) -> String {
    // first attempt: allocates a vector and a string
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

#[no_mangle]
pub extern "C" fn summarize_html(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }

    // Convert C string pointer to Rust string
    let c_str = unsafe { CStr::from_ptr(input) };
    let input_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let parser = kuchiki::parse_html().one(input_str);
    parser
        .inclusive_descendants()
        .filter(|node| {
            node.as_element().map_or(false, |e| {
                matches!(e.name.local.as_ref(), "script" | "style" | "noscript")
            })
        })
        .collect::<Vec<_>>()
        .iter()
        .for_each(|node| node.detach());

    let text = trim_whitespace(&parser.text_contents());
    let result_str = _summarize_text(text.as_str(), Some(30));
    // Convert the Rust string back to a C string and return the pointer
    let result_c_string = CString::new(result_str.join("")).unwrap();
    result_c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn summarize_text(input: *const c_char) -> *mut c_char {
    // Safety: Ensure that the input pointer is not null
    if input.is_null() {
        return std::ptr::null_mut();
    }
    // Convert C string pointer to Rust string
    let c_str = unsafe { CStr::from_ptr(input) };
    let input_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let result_str = _summarize_text(input_str, None);

    // Convert the Rust string back to a C string and return the pointer
    let result_c_string = CString::new(result_str.join("")).unwrap();
    result_c_string.into_raw()
}

// Function to deallocate the memory allocated for the returned C string
#[no_mangle]
pub extern "C" fn free_memory(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

fn _summarize_text(input: &str, max_tokens: Option<usize>) -> Vec<String> {
    let max_tokens = max_tokens.unwrap_or(150);

    let prompt = "summarize:".to_string() + input;

    let (builder, mut tokenizer) =
        T5ModelBuilder::load(MODEL_NAME.to_string(), "main".to_string()).unwrap();

    let device = &builder.device;
    let tokenizer = tokenizer
        .with_padding(None)
        .with_truncation(None)
        .map_err(E::msg)
        .unwrap();

    let tokens = tokenizer
        .encode(prompt, true)
        .map_err(E::msg)
        .unwrap()
        .get_ids()
        .to_vec();

    let input_token_ids = Tensor::new(&tokens[..], device)
        .unwrap()
        .unsqueeze(0)
        .unwrap();
    let mut model = builder.build_model().unwrap();

    let mut output_token_ids = [builder
        .config
        .decoder_start_token_id
        .unwrap_or(builder.config.pad_token_id) as u32]
    .to_vec();

    let mut logits_processor = LogitsProcessor::new(299792458, None, None);
    let encoder_output = model.encode(&input_token_ids).unwrap();
    let start = std::time::Instant::now();

    let mut result_str = vec![];
    let mut generated_tokens = 0;

    for index in 0.. {
        if output_token_ids.len() > 512 {
            break;
        }
        let decoder_token_ids = if index == 0 || !builder.config.use_cache {
            Tensor::new(output_token_ids.as_slice(), device)
                .unwrap()
                .unsqueeze(0)
                .unwrap()
        } else {
            let last_token = *output_token_ids.last().unwrap();
            Tensor::new(&[last_token], device)
                .unwrap()
                .unsqueeze(0)
                .unwrap()
        };
        let logits = model
            .decode(&decoder_token_ids, &encoder_output)
            .unwrap()
            .squeeze(0)
            .unwrap();

        let next_token_id = logits_processor.sample(&logits).unwrap();
        if next_token_id as usize == builder.config.eos_token_id {
            break;
        }

        output_token_ids.push(next_token_id);
        if let Some(text) = tokenizer.id_to_token(next_token_id) {
            let text = text.replace('▁', " ").replace("<0x0A>", "\n");
            print!("{text}");
            generated_tokens += 1;
            std::io::stdout().flush().unwrap();
            result_str.push(text);
            if generated_tokens >= max_tokens {
                break;
            }
        }
    }
    let dt = start.elapsed();
    println!(
        "\n{} tokens generated ({:.2} token/s)\n",
        output_token_ids.len(),
        output_token_ids.len() as f64 / dt.as_secs_f64(),
    );
    result_str
}

pub fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    Ok(
        v.broadcast_div(&v.sqr().unwrap().sum_keepdim(1).unwrap().sqrt().unwrap())
            .unwrap(),
    )
}
