Glicol stands for graph-oriented live coding language.

It is developed with Rust, WebAssembly and AudioWorklet.

To test it:

```
cd glicol-wasm
cargo build --target wasm32-unknown-unknown --release
cd ../glicol-js
yarn
sudo zsh test.sh  // or run test.bat on windows
```