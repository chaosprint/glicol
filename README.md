<div align="center">
  <br />
  <p>
    <a href="https://glicol.org"><img src="./logo.png" width="200" /></a>
  </p>
</div>

GLICOL (an acronym for "graph-oriented live coding language") is a computer music language and an audio DSP library written in Rust.

## Why Glicol?
Glicol is mainly contributing to these two domains:

1. rethink language style and interaction in collaborative live coding
2. as an audio lib for quick DSP prototyping

### Language style and interaction

As its name suggests, Glicol opts a graph-oriented syntax, rather than OOP or FP.
The consideration is from balancing the simplicity and readability.
Thus, the programming in Glicol is all about:
1. remembering the input and output of each node
2. connect them

```
// an amplitude modulation example
o: sin 440 >> mul ~mod

~mod: sin 0.5 >> mul 0.3 >> add 0.5
```

For interaction, Glicol choose a WYSIWYG (what-you-see-is-what-you-get) paradigm. Under the hood, Glicol has implemented LCS algorithm to dynamically update the graph in real-time. Together with code preprocessing, Glicol provides smooth transition for oscillators and the `mul` node.

You can learn Glicol, find music example and create decentralised collaboration on its web interface:

https://glicol.org

The web interface has the following features:
1. run Glicol engine at near-native speed, thanks to WebAssembly
2. garbage-collection-free real-time audio in browsers thanks to AudioWorklet, SharedArrayBuffer
3. error handling and command in browser console: e.g. load your own samples
4. mix JS code with Glicol easily: `o: sin {42*10+20}`
5. create visuals with Hydra

### DSP

Glicol can be used:

1. as Web Audio API, Tone.js alternative
2. for developing VST plugins in Rust (has POC, but still WIP)
3. to program on Bela board (has POC, but still WIP)

The `js` folder contains the Glicol distribution for the web platform.
The usage is very easy. Just include this into your `index.html`:
```
<script src="https://cdn.jsdelivr.net/gh/chaosprint/glicol@latest/js/src/glicol.js"></script>
```
Then you can write on your website:
```
run(`o: sin 440`)
```

See the [README.md](./js/README.md) in `js` folder for details.

For VST plugins development, see the [README.md](./rs/README.md) file in the `rs` folder for details.

## Features, milestones and status quo

- [x] `0.1.0` Hello world from `dasp_graph` and `pest.rs`, pass code from JS to WASM, and lazy evaluation
- [x] `0.2.0` Pass samples from JS to WASM, support error handling, BPM control in console
- [x] `0.3.0` Build complex node `plate` reverb using basic node from Glicol, using Macro in Rust
- [x] `0.4.0` Apply LCS and preprocessor for smooth and efficient whole graph updating
- [x] `0.5.0` Build `const_generics` to `dasp_graph` and use it in Glicol, use `SharedArrayBuffer`, support local sample loading
- [x] `0.6.0` Refactor the code to modules: 
    - `glicol-main` = `glicol-synth` + `glicol-parser` + `glicol-ext`
    - `glicol-ext` = `glicol-synth` + `glicol-parser` + `glicol-macro`
    - `glicol-js` = `glicol-main` + `glicol-wasm`
- [x] `0.7.0` Support mixing JS with Glicol in `glicol-js` using Regex
- [ ] `0.8.0` Detailed and robust error handling.
- [ ] `0.9.0` Better node, VST, Web Audio development protocol
- [ ] `1.0.0` Finalise docs, music examples and fix bugs

Please let me know in issues or discussion:
- new features suggestion
- bug report
- missing and confusion in any docs and guides