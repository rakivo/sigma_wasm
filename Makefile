BUILD_DIR_FLAG = --target-dir build

all: native web

native: game.rs native.rs
	RUSTFLAGS="-L./lib/ -lraylib" cargo build --features="native" $(BUILD_DIR_FLAG)

web: game.rs
	cargo build --features="web" --target=wasm32-unknown-unknown $(BUILD_DIR_FLAG)
