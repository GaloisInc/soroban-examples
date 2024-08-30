default: build

all: test

test: build
	cargo test

build:
	cargo build --target wasm32-unknown-unknown --release 
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

# We need a wasm build as well as a crucible/json build, because
# the build of contract_b needs to examine the output as part of
# the cross-call machinery. So always do a regular build as well.
saw-build:
	$(MAKE) build
	cargo saw-build --target wasm32-unknown-unknown --release
	cp target/*/release/deps/*.linked-mir.json target/linked-mir.json

fmt:
	cargo fmt --all

clean:
	cargo clean
