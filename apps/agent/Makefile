SRC_DIR := $(shell git rev-parse --show-toplevel)
PKG_DIR := $(SRC_DIR)/package
CUR_DIR := $(shell pwd)
GRADLE := $(SRC_DIR)/gradlew
SERVER_SDK_DST := $(PKG_DIR)/geth-agent-server
CLIENT_SDK_DST := $(PKG_DIR)/geth-agent-client
SERVER_SDK_SRC := $(SRC_DIR)/models/agent/build/smithyprojections/agent/proj/rust-server-codegen
CLIENT_SDK_SRC := $(SRC_DIR)/models/agent/build/smithyprojections/agent/proj/rust-client-codegen

all: build

codegen:
	cd ${SRC_DIR}/models/agent && gradle build
	mkdir -p $(SERVER_SDK_DST) $(CLIENT_SDK_DST)
	cp -av $(SERVER_SDK_SRC)/* $(SERVER_SDK_DST)/
	cp -av $(CLIENT_SDK_SRC)/* $(CLIENT_SDK_DST)/

build: codegen
	cargo build

run: codegen
	cargo run

clippy: codegen
	cargo clippy

test: codegen
	cargo test

doc-open: codegen
	cargo doc --no-deps --open

clean:
	cargo clean || echo "Unable to run cargo clean"

distclean: clean
	rm -rf $(SERVER_SDK_DST) $(CLIENT_SDK_DST) Cargo.lock

.PHONY: all