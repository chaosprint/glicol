cargo build --target wasm32-unknown-unknown --release

cp -f ./public/mode-quaver.js ./node_modules/ace-builds/src-noconflict/
cp -f ./public/theme-quaver-night.js ./node_modules/ace-builds/src-noconflict/
mv -f ./target/wasm32-unknown-unknown/release/quaverseries_wasm.wasm ./public/wasm/