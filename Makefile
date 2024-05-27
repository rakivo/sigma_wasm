all: raylib.wasm wasm.wasm

raylib.wasm: raylib/raylib.rs
	rustc --crate-type=rlib --target=wasm32-unknown-unknown $<

wasm.wasm: lib.rs
	rustc --crate-type=cdylib --target=wasm32-unknown-unknown --extern raylib=./libraylib.rlib -o $@ $<
