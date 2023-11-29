#[cfg(feature = "accelerate")]
extern crate accelerate_src;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use candle_transformers::models::t5;

use anyhow::{Error as E, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

const DTYPE: DType = DType::F32;

struct T5ModelBuilder {
    device: Device,
    config: t5::Config,
    weights_filename: Vec<PathBuf>,
}

impl T5ModelBuilder {
    pub fn load(model_id: String, revision: String) -> Result<(Self, Tokenizer)> {
        let device = Device::Cpu;
        let repo = Repo::with_revision(model_id.clone(), RepoType::Model, revision);
        let api = Api::new()?;
        let api = api.repo(repo);
        let config_filename = api.get("config.json")?;
        let tokenizer_filename = api.get("tokenizer.json")?;
        let weights_filename = if model_id == "google/flan-t5-xxl" {
            vec![
                api.get("model-00001-of-00005.safetensors")?,
                api.get("model-00002-of-00005.safetensors")?,
                api.get("model-00003-of-00005.safetensors")?,
                api.get("model-00004-of-00005.safetensors")?,
                api.get("model-00005-of-00005.safetensors")?,
            ]
        } else if model_id == "google/flan-ul2" {
            vec![
                api.get("model-00001-of-00008.safetensors")?,
                api.get("model-00002-of-00008.safetensors")?,
                api.get("model-00003-of-00008.safetensors")?,
                api.get("model-00004-of-00008.safetensors")?,
                api.get("model-00005-of-00008.safetensors")?,
                api.get("model-00006-of-00008.safetensors")?,
                api.get("model-00007-of-00008.safetensors")?,
                api.get("model-00008-of-00008.safetensors")?,
            ]
        } else {
            vec![api.get("model.safetensors")?]
        };
        let config = std::fs::read_to_string(config_filename)?;
        let mut config: t5::Config = serde_json::from_str(&config)?;
        config.use_cache = true;
        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;
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
            VarBuilder::from_mmaped_safetensors(&self.weights_filename, DTYPE, &self.device)?
        };
        Ok(t5::T5ForConditionalGeneration::load(vb, &self.config)?)
    }
}

fn read_file_to_string(file_path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn main() -> Result<()> {
    let prompt = "summarize:".to_string() + &read_file_to_string("sandman.txt")?;
    let (builder, mut tokenizer) =
        T5ModelBuilder::load("t5-small".to_string(), "main".to_string())?;

    let device = &builder.device;
    let tokenizer = tokenizer
        .with_padding(None)
        .with_truncation(None)
        .map_err(E::msg)?;

    let tokens = tokenizer
        .encode(prompt, true)
        .map_err(E::msg)?
        .get_ids()
        .to_vec();
    let input_token_ids = Tensor::new(&tokens[..], device)?.unsqueeze(0)?;
    let mut model = builder.build_model()?;

    let mut output_token_ids = [builder
        .config
        .decoder_start_token_id
        .unwrap_or(builder.config.pad_token_id) as u32]
    .to_vec();

    let mut logits_processor = LogitsProcessor::new(299792458, None, None);
    let encoder_output = model.encode(&input_token_ids)?;
    let start = std::time::Instant::now();

    for index in 0.. {
        if output_token_ids.len() > 512 {
            break;
        }
        let decoder_token_ids = if index == 0 || !builder.config.use_cache {
            Tensor::new(output_token_ids.as_slice(), device)?.unsqueeze(0)?
        } else {
            let last_token = *output_token_ids.last().unwrap();
            Tensor::new(&[last_token], device)?.unsqueeze(0)?
        };
        let logits = model
            .decode(&decoder_token_ids, &encoder_output)?
            .squeeze(0)?;

        let next_token_id = logits_processor.sample(&logits)?;
        if next_token_id as usize == builder.config.eos_token_id {
            break;
        }

        output_token_ids.push(next_token_id);
        if let Some(text) = tokenizer.id_to_token(next_token_id) {
            let text = text.replace('▁', " ").replace("<0x0A>", "\n");
            print!("{text}");
            std::io::stdout().flush()?;
        }
    }
    let dt = start.elapsed();
    println!(
        "\n{} tokens generated ({:.2} token/s)\n",
        output_token_ids.len(),
        output_token_ids.len() as f64 / dt.as_secs_f64(),
    );
    Ok(())
}

pub fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}
