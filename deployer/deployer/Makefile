default: build

all: test

test: build
	cargo test

build:
	$(MAKE) -C ../contract || break;
	cargo build --target wasm32-unknown-unknown --release 
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

saw-build:
	$(MAKE) -C ../contract saw-build || false
	cargo saw-build --target wasm32-unknown-unknown --release
	cp target/*/release/deps/*.linked-mir.json target/linked-mir.json

fmt:
	cargo fmt --all

clean:
	cargo clean
