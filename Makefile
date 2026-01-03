SOURCES := $(shell find src -name '*.rs') Cargo.toml Cargo.lock

target/wasm32-unknown-unknown/release/ray_tracer_wasm.wasm: $(SOURCES)
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/ray_tracer_wasm.wasm web/ray_tracer_wasm.wasm