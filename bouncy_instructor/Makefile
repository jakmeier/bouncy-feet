# Set target dir to default if not set in env
CARGO_TARGET_DIR ?= ../target

all:
	wasm-pack build --target web
	sort ./pkg/bouncy_instructor_bg.wasm.d.ts -o ./pkg/bouncy_instructor_bg.wasm.d.ts
	cp -r pkg/* ../bouncy_frontend/src/lib/instructor/

debug:
	RUSTFLAGS=-g cargo build --release --target=wasm32-unknown-unknown
	wasm-bindgen ${CARGO_TARGET_DIR}/wasm32-unknown-unknown/release/bouncy_instructor.wasm --target --out-dir pkg
	sort ./pkg/bouncy_instructor_bg.wasm.d.ts -o ./pkg/bouncy_instructor_bg.wasm.d.ts
	cp -r pkg/* ../bouncy_frontend/src/lib/instructor/
