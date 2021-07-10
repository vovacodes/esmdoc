build:
	wasm-pack build --release --out-dir generated --target web
	# Patch generated/package.json
	deno run --allow-read=generated/package.json --allow-write=generated/package.json build_scripts/patch_package_json.ts

lint:
	cargo clippy -- -D warnings

size: build
	gzip -c generated/esmdoc_bg.wasm | wc -c

test:
	cargo test

test-wasm: build
	deno test --allow-read wasm_tests/tests.ts

test-integration:
	wasm-pack test --headless --safari

.PHONY: build lint size test test-integration
