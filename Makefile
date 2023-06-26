SRC_DIR := $(shell git rev-parse --show-toplevel)

agent:
	cd apps/agent && make

agent-build:
	cd apps/agent && make build

agent-install:
	cd apps/agent && make install

control:
	cd apps/control && make

control-codegen:
	cd apps/control && make codegen