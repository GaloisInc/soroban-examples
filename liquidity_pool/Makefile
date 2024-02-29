default: build

all: test

test: build
	cargo test

build:
	$(MAKE) -C ../token || break;
	cargo build --target wasm32-unknown-unknown --release 
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

saw-build:
	$(MAKE) -C ../token saw-build || false
	cargo saw-build --target wasm32-unknown-unknown --release
	cp target/*/release/deps/*.linked-mir.json target/linked-mir.json

fmt:
	cargo fmt --all

clean:
	cargo clean
