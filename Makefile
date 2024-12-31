.PHONY: all
MAKEFLAGS += --silent

all: help

help:
	@grep -E '^[a-zA-Z1-9\._-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| sort \
		| sed -e "s/^Makefile://" -e "s///" \
		| awk 'BEGIN { FS = ":.*?## " }; { printf "\033[36m%-30s\033[0m %s\n", $$1, $$2 }'

build: ## Build the wasi component
	wit-deps
	cargo build --target wasm32-wasip2 --release
	cp ./target/wasm32-wasip2/release/meta_capi_component.wasm meta_capi.wasm 

test: ## Test the component on host platform
	cargo test --lib