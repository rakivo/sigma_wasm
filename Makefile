BUILD_DIR_FLAG = --target-dir ../build

all: native web

native: examples/funny_rect.rs examples/native.rs
	cd examples && RUSTFLAGS="-L../lib/ -lraylib" cargo build --features="native" $(BUILD_DIR_FLAG)

web: examples/funny_rect.rs
	cd examples && cargo build --features="web" --target=wasm32-unknown-unknown $(BUILD_DIR_FLAG)

run_native: examples/funny_rect.rs examples/native.rs
	cd examples && RUSTFLAGS="-L./lib/ -lraylib" cargo run --features="native" $(BUILD_DIR_FLAG)
