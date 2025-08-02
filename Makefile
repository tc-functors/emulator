all: build

TARGET_DIR=$(HOME)/.cargo/target
BIN_DIR=bin
PROJECT_VERSION=$(shell git rev-parse --short HEAD)

build:
	@mkdir -p $(BIN_DIR)
	cargo build
	@cp $(TARGET_DIR)/debug/tc-emulator tc-emulator

fmt:
	cargo +nightly fmt

clean:
	rm -rf bin tc-emulator
