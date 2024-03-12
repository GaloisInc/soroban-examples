default: build

all: test

test: build
	cargo test

build:
	cargo build --target wasm32-unknown-unknown --release 
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

# Build the version that actually generates log messages so
# something happens
saw-build:
	cargo saw-build --target wasm32-unknown-unknown \
		--profile release-with-logs
	cp target/*/release-with-logs/deps/*.linked-mir.json \
		target/linked-mir.json

fmt:
	cargo fmt --all

clean:
	cargo clean
