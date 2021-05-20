# Introduction

Glicol is a **g**raph-based **li**ve **co**ding **l**anguage written in Rust.

The easist way to try it is via the browser, thanks to WebAssembly, AudioWorklet and SharedArrayBuffer.

See:

[glicol.web.app](https://glicol.web.app)

# Usage

This folder contains the Rust source code of Glicol, with the language and DSP modules wrapped as one `Engine` struct.

In the `exmaple` folder there are two examples showing how to use Glicol APIs in Rust.

You should install gnuplot on your OS before you run:
```
cargo run --example plot
```
Or:
```
cargo run --example update
```

# License

The MIT License (MIT)

Copyright (c) 2020 - present Qichao Lan (chaosprint)