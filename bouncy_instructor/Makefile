all:
	cargo build --release --target=wasm32-unknown-unknown
	wasm-bindgen $${CARGO_TARGET_DIR}/wasm32-unknown-unknown/release/bouncy_instructor.wasm --out-dir pkg
	sort ./pkg/bouncy_instructor_bg.wasm.d.ts -o ./pkg/bouncy_instructor_bg.wasm.d.ts
	cp -r pkg/* ../bouncy_frontend/src/lib/instructor/