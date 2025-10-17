.PHONY: all cpp-bench build-cpp exec-cpp rust-bench build-rust exec-rust clean

all: cpp-bench rust-bench

cpp-bench: build-cpp exec-cpp

build-cpp:
	mkdir -p build/cpp
	g++ -std=c++17 -O2 -Wall -Wextra -pedantic -o build/cpp/main vector_deque_list.cpp
	cp build/cpp/main main

exec-cpp:
	./build/cpp/main

rust-bench: build-rust exec-rust

build-rust:
	mkdir -p build/rust
	rustc -C opt-level=2 -o build/rust/main vector_deque_list.rs
	cp build/rust/main main

exec-rust:
	./build/rust/main

clean:
	rm -rf build main
