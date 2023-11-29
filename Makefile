.PHONY: build build-rust build-cpp


run:
	./moz_genai

build: build-rust build-cpp

build-rust:
	cargo build --release

build-cpp:
	cd cpp; g++ -o ../moz_genai main.cpp -I../target -L../target/release -lmoz_genai

