# How to build the Bouncy Feet app for developers

The following commands have been tested in a bash shell on a Linux system only.
Other systems may work, too. But perhaps they need small tweaks.

## Build tools for Bouncy Instructor (Rust + Wasm)
1. Install Rust: https://rustup.rs/
2. Add Wasm target
  rustup target add wasm32-unknown-unknown
3. Install Wasm-bindgen tooling
  cargo install wasm-bindgen-cli
4. Try running `make` in ./bouncy_instructor
  If something fails, check steps above, otherwise move on.

## Tools for frontend (Svelte)
5. Install node
  - personally I use https://github.com/nvm-sh/nvm to manage node versions
6. change directory to ./bouncy_frontend
  - `npm install`
  - `npm run dev`
7. Open the localhost link fromthe previous command to view the app.
  - in the JS console, type `toggleDev()` to turn on/off developer features

## General dev workflow
- Start dev server, it will listen to live changes in JS code
  - `npm run dev` in ./bouncy_frontend
- If changes were made to Rust code, you need to manually trigger compilation
  - `make` in ./bouncy_instructor
