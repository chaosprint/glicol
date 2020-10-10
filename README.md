This lib is for creating WebAssembly file for Glicol live coding language, which can later be running in browsers.

To test it, clone this repository together with [github.com/glicol/glicol-rs](github.com/glicol/glicol-rs) and put thees two projects in the same folder.

```
git clone https://github.com/glicol/glicol-rs.git
git clone https://github.com/glicol/glicol-wasm.git
cd glicol-wasm
cargo build --target wasm32-unknown-unknown --release
```

Visit [github.com/glicol/glicol-js](github.com/glicol/glicol-js) for the next step to run it in the browser.