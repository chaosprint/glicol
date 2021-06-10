# Introduction

Glicol (graph-oriented live coding language) is a computer music language written in Rust.

Despite its versatility, Glicol is currently targeting browser-based collaborative live coding.

# Why Glicol?

***Glicol is fast.*** Glicol has both its language and DSP engines written in Rust. With the zero-cost abstractions in Rust, Glicol is very fast.

***Glicol is reliable.*** Glicol has reliable error-handling strategies written in Rust, so you do not need to worry about typos in live performances. If an error is detected, the engine will continue playing the previous error-free codes.

***Glicol is easy to access.*** Using WebAssembly, AudioWorklet and SharedArrayBuffer, browsers can now have lock-free real-time audio processing capacity. Glicol has used all these technologies to provide a zero-installation experience. Just visit the website and you can start live coding.

***Glicol is interaction-friendly.*** Just run the code. What you see on screen is what you hear.
Commenting out a line of code is equivalent to muting a track.

***Glicol is intuitive.*** Glicol uses a graph-oriented syntax, bypassing paradigms such as OOP or FP. For example, you can synthesise kick drum like this:

```
bd: sin ~pitch >> mul ~env >> mul 0.9

~trigger: speed 4.0 >> seq 60

~env: ~trigger >> envperc 0.01 0.4

~env_pitch: ~trigger >> envperc 0.01 0.1

~pitch: ~env_pitch >> mul 80 >> add 60
```

Just like playing module synth.

Play with the code at: https://glicol.web.app/4CY8UM

# Where to start?

Glicol has launched its official website at: 

https://glicol.org

Still, the old web app will remain as the playground:

https://glicol.web.app

*On both website, opening the browser console is important as the helps and some key commands are exported there.*

# Repository structure

```
js/
├─ src/
│  ├─ glicol_wasm.wasm
│  ├─ glicol-docs.json
│  ├─ glicol-engine.js
│  ├─ glicol.js
├─ index.html
rs/
├─ ext/
├─ macro/
├─ main/
├─ parser/
├─ synth/
├─ wasm/
```

## Rust

The `rs` folder contains the Rust code for Glicol.

The `rs/main` is the main entrance of crate `glicol`.

The `rs/parser` is the `glicol_parser` crate, which provides the parsing tool for Glicol syntax.

The `rs/synth` is the `glicol_synth` crate, which contains the DSP code for Glicol and can be used as an independent audio lib.

The `rs/macro` provides Rust macros for developing Glicol extensions.

The `rs/ext` is the Glicol extensions, which replies on `glicol_synth`, `glicol_parser` and `glicol_macro`. The idea is to use the essential nodes in `glicol_synth` to form some more complicated nodes, e.g. reverb nodes. Developers can use the `glicol_macro` to write new node in Glicol syntax within Rust.
```
glicol_ext = glicol_synth + glicol_parser + glicol_macro
```
```
glicol = glicol_parser + glicol_synth + glicol_ext
```
The `rs/wasm` is basically the glue code for compiling the `glicol` crate into a WebAssembly file.

## JavaScript

The `js` folder contains the Glicol distribution for the web platform.

The usage is very easy. Just include this into your `index.html`:

```
<script src="https://cdn.jsdelivr.net/gh/chaosprint/glicol@latest/js/src/glicol.js"></script>
```

See the `README.md` in `js` folder for details.

# Contribution

Suggestions, bug reporting, or PR are warmly welcomed.

# Acknowledgement

This work was partially supported by the Research Council of Norway through its Centres of Excellence scheme, project number 262762 and by NordForsk's Nordic University Hub Nordic Sound and Music Computing Network NordicSMC, project number 86892.

# License

The MIT License (MIT)

Copyright (c) 2020 - present Qichao Lan (chaosprint)