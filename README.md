This lib is for creating WebAssembly file for Glicol live coding language, which can later be used in the browsers.

To test it, clone this repository together with [github.com/glicol/glicol-rs](github.com/glicol/glicol-rs) and put thees two projects in the same folder.

```
git clone https://github.com/glicol/glicol-rs.git
git clone https://github.com/glicol/glicol-wasm.git
cd glicol-wasm
cargo build --target wasm32-unknown-unknown --release
```