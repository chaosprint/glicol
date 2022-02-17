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
4. mix JS code with Glicol easily: `o: sin {{42*10+20}}`
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

## Features and milestones

- [x] `0.1.0` hello world from `dasp_graph` and `pest.rs`, pass code from js to wasm, and lazy evaluation
- [x] `0.2.0` pass samples from js to wasm, support error handling, bpm control in console
- [x] `0.3.0` build complex node `plate` reverb using basic node from glicol, using macro in Rust
- [x] `0.4.0` use `LCS` algorithm and preprocessor for smooth and efficient whole graph updating
- [x] `0.5.0` build `const_generics` to `dasp_graph` and use it in glicol, use `SharedArrayBuffer`, support local sample loading
- [x] `0.6.0` refactor the code to modules: 
    - `glicol-main` = `glicol-synth` + `glicol-parser` + `glicol-ext`
    - `glicol-ext` = `glicol-synth` + `glicol-parser` + `glicol-macro`
    - `glicol-js` = `glicol-main` + `glicol-wasm`
- [x] `0.7.0` support mixing js with glicol in `glicol-js` using Regex; add visualisation
- [ ] `0.8.0` embed `Rhai` in glicol 🎉
```
main: script "
    output.clear();
    for i in 0..128 {
        output.push(sin(2.0*PI()*phase / (44100.0 / 55.0 )));
        phase += 1.0;
    };
    output
" >> script "
    output = input.map(|i|i*0.1);
    output
"
```
- [ ] `0.9.0` better docs/tutorials, music examples and bug fix
- [ ] detailed and robust error handling; may swtich to `nom` parser
- [ ] midi support? better communication between wasm and js
- [ ] better extension node, vst, web audio development protocol 

> Note that Glicol is still highly experimental, so it can be highly risky for live performances. The API may also change before version 1.0.0.

Please let me know in [issues](https://github.com/chaosprint/glicol/issues) or [discussions](https://github.com/chaosprint/glicol/discussions):
- your thoughts and experience with glicol
- new features suggestion
- bug report
- missing and confusion in any docs and guides