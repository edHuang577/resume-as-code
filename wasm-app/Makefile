.PHONY: build serve

all: build serve

build:
	wasm-pack build --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js

serve:
	python -m http.server 8080
