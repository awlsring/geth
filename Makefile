SRC_DIR := $(shell git rev-parse --show-toplevel)

all: codegen

agent:
	cd apps/agent && make

agent-build:
	cd apps/agent && make build