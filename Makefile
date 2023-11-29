.PHONY: build build-rust build-cpp


run:
	./summarizer

build: build-rust build-cpp

build-rust:
	cargo build --release

build-cpp:
	cd cpp; g++ -o ../summarizer main.cpp -I../output -L../target/release -lsummary

