
build-rust:
	cargo build --release

build-cpp:
	cd cpp; g++ -o ../summarizer main.cpp -I../output -L../target/release -lsummary

