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
	@cp $(TARGET_DIR)/release/orb $(BIN_DIR)/

clean:
	rm -rf bin
