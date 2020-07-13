cargo build --target wasm32-unknown-unknown --release

mv -f ./target/wasm32-unknown-unknown/release/quaver.wasm ./public/wasm/