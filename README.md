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

# Repository structure and development

This repository has three folders:
- ```glicol-rs```
- ```glicol-wasm```
- ```glicol-js```

The ```glicol-rs``` folder contains the main Rust code for the language parsing and DSP engine.
The ```glicol-wasm``` folder is for compiling the Rust code into a WebAssembly module.
The ```glicol-js``` contains the JS code, which can be used as a CDN link.

See the README on each folder for details.

# Contribution

Suggestions, bug reporting, or PR are warmly welcomed.

# Acknowledgement

https://crates.io/crates/dasp_graph

# License

The MIT License (MIT)

Copyright (c) 2020 - present Qichao Lan (chaosprint)