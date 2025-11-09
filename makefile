.PHONY: build clean run

PROJECT_NAME := PA2

run: build
	./target/debug/$(PROJECT_NAME)

build: clean
	cargo build

clean:
	cargo clean
