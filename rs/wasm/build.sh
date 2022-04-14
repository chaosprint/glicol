cargo build --target wasm32-unknown-unknown --release
cp -rf ./target/wasm32-unknown-unknown/release/glicol_wasm.wasm ../../js/src
cp -rf ./target/wasm32-unknown-unknown/release/glicol_wasm.wasm ../../js/npm