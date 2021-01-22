# Introduction
Glicol is a graph-oriented live coding language developed with Rust, WebAssembly and AudioWorklet.

It is currently running on its collaborative web-based IDE with documentation and tutorials built in the browser console:

https://glicol.web.app

Feel free to try it as a live coding language or a tool for quick audio effect prototyping. If you find bugs, have any questions, or simply hope to add some features for your project, please open new issues and pull request are welcomed.

# Repository structure and development

This repository has three folders:
- ```glicol-rs```
- ```glicol-wasm```
- ```glicol-js```

The ```glicol-rs``` folder contains the main Rust code for the language parser and audio engine.
The ```glicol-wasm``` folder is for compiling the Rust code into a WebAssembly module.
The ```glicol-js``` contains the JS code for the collaborative web-based IDE and the AudioWorklet engine.


```
cd glicol-rs
zsh plot.sh // or `plot.bat` on windows
```

```
cd glicol-wasm
cargo build --target wasm32-unknown-unknown --release
```

```
cd glicol-js
yarn
zsh test.sh  // or `test.bat` on windows
```

# Acknowledgement
https://pest.rs/

https://crates.io/crates/dasp_graph

https://firepad.io/

# License
The MIT License (MIT)

Copyright (c) 2020 Qichao Lan