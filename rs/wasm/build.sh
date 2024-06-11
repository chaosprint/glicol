cargo build --target wasm32-unknown-unknown --release
cp -rf ./target/wasm32-unknown-unknown/release/glicol_wasm.wasm ../../js/src
cp -rf ./target/wasm32-unknown-unknown/release/glicol_wasm.wasm ../../js/npm

wasm-opt ../../js/src/glicol_wasm.wasm -Oz -o ../../js/src/glicol_wasm.wasm
wasm-opt ../../js/npm/glicol_wasm.wasm -Oz -o ../../js/npm/glicol_wasm.wasm