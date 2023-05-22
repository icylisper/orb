all: build

TARGET_DIR=$(HOME)/.cargo/target
BIN_DIR=bin

build:
	mkdir -p $(BIN_DIR)
	cargo build
	@cp $(TARGET_DIR)/debug/orb $(BIN_DIR)/

release:
	mkdir -p $(BIN_DIR)
	cargo build --release
	@cp $(TARGET_DIR)/release/orb $(BIN_DIR)/orb-x86_64-linux

clean:
	rm -rf bin

export PATH := $(HOME)/opt/apple/osxcross/target/bin:$(PATH)
export CC=o64-clang
export CXX=o64-clang++
export LIBZ_SYS_STATIC=1

release-osx:
	cargo build --release --target x86_64-apple-darwin
	@cp $(TARGET_DIR)/x86_64-apple-darwin/release/orb $(BIN_DIR)/orb-x86_64-apple
