<div align="center">
  <br />
  <p>
    <a href="https://glicol.org"><img src="./logo.png" width="200" /></a>
  </p>
</div>

GLICOL (an acronym for "graph-oriented live coding language") is a computer music language and an audio DSP library written in Rust.

## Why Glicol?
Top 5 features:
### 1. Graph-oriented syntax
As its name suggests, Glicol opts a graph-oriented syntax, rather than OOP or FP.
The consideration is from balancing the simplicity and readability.
Thus, all you need to know is the connection and the io range.
You can view each node as synth module that you can literally hear/see.

```
// amplitude modulation example
a: sin 440 >> mul ~mod // lazy evaluation
~mod: sin 0.2 >> mul 0.5 >> add 0.5 // ~mod is a ref
// names with ~ will be not sent to dac
```

```
// sequencer pattern
// first divide one bar with space
// then further divide each part
// _ means rest
o: speed 2.0 >> seq 60 _~a _ 60__60
>> sp \blip
~a: choose 60 60 0 0 72 72 // quantity alters probability
```

### 2. What you see is what you get
For interaction, Glicol choose a WYSIWYG (what-you-see-is-what-you-get) paradigm. Under the hood, Glicol has implemented LCS algorithm to dynamically update the graph in real-time.
### 3. Zero-installation

You can learn Glicol, find music example and create decentralised collaboration on its web interface:

https://glicol.org

The web interface has the following features:
1. run Glicol engine at near-native speed, thanks to WebAssembly
2. garbage-collection-free real-time audio in browsers thanks to AudioWorklet, SharedArrayBuffer
3. error handling and command in browser console: e.g. load your own samples
4. mix JS code with Glicol easily: `o: sin ##42*10+20#`
5. create visuals with Hydra

### 4. Rust audio

Glicol has its own audio library `glicol_synth` and can be used:

1. as Web Audio API, Tone.js alternative
2. for developing VST plugins in Rust (has POC, but still WIP)
3. to program on Bela board (has POC, but still WIP)

The `js` folder contains the Glicol distribution for the web platform. The usage is very easy. Just include this into your `index.html`:
```
<script src="https://cdn.jsdelivr.net/gh/chaosprint/glicol@latest/js/src/glicol.js"></script>
```
Then you can write on your website:
```
run(`o: sin 440`)
```

See the [README.md](./js/README.md) in `js` folder for details.

For Rust audio, see the [README.md](./rs/README.md) file in the `rs` folder for details.

### 5. One more thing
You can use `script` to write meta node, which is like the `gen~` in Max/MSP.

```
// a sawtooth osc chained with a onepole filter
a: script `
	f = 220.;
	output.pad(128, 0.0);
	if phase == 0 {
		p = 0.0;
	}
	for i in 0..128 {
		output[i] = p * 2. - 1.;
		p += f / sr;
	};
	if p > 1.0 { p -= 1.0 };
	output
` >> script `
	r = 1./2000.;
	if phase == 0.0 {
		z = 0.0
	}
	output.pad(128, 0.0);
	b = (-2.0 * PI() * r).exp();
	a = 1.0 - b;
	for i in 0..128 {
		y = input[i] * a + b * z;
		output[i] = y;
		z = y;
	};
	output
`
```

## Milestones

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
- [x] `0.8.0` embed `Rhai` in glicol 🎉
```
main: script `
    output.clear();
    for i in 0..128 {
        output.push(sin(2.0*PI()*phase / (44100.0 / 55.0 )));
        phase += 1.0;
    };
    output
` >> script `
    output = input.map(|i|i*0.1);
    output
`
```
- [x] `0.9.0` robust error handling; redesigned architecture. see the release note
- [ ] better docs/tutorials, music examples and bug fix
- [ ] midi support? better communication between wasm and js
- [ ] better extension node, vst, web audio development protocol 

> Note that Glicol is still highly experimental, so it can be risky for live performances. The API may also change before version 1.0.0.

Please let me know in [issues](https://github.com/chaosprint/glicol/issues) or [discussions](https://github.com/chaosprint/glicol/discussions):
- your thoughts on the experience of glicol
- new feature suggestion
- bug report, especially the code that causes a `panic` in browser console
- missing and confusion in guides and reference on the website