<div align="center">
  <br />
  <p>
    <a href="https://glicol.org"><img src="https://github.com/chaosprint/glicol/raw/main/logo.png" width="200" /></a>
  </p>
</div>

<div align="center">
  <a href="https://glicol.org" target="_blank"><img alt="website" src="https://img.shields.io/badge/website-glicol.org-blue"></a>
  <a href="https://glicol.js.org" target="_blank"><img alt="website" src="https://img.shields.io/badge/npm%20docs-glicol.js.org-yellow"></a>
  <a href="https://npmjs.com/glicol" target="_blank"><img alt="npm" src="https://img.shields.io/npm/v/glicol"></a>
  <a href="https://discord.gg/8tmK2bHcwa" target="_blank"><img alt="Discord" src="https://img.shields.io/discord/963514061528662046"></a>
  <a href="https://github.com/chaosprint/glicol/blob/main/LICENSE"><img alt="GitHub" src="https://img.shields.io/github/license/chaosprint/glicol"></a>
</div>

Glicol (an acronym for "graph-oriented live coding language") is a computer music language with both its language and audio engine written in [Rust programming language](https://www.rust-lang.org/), a modern alternative to C/C++. Given this low-level nature, Glicol can run on many different platforms such as browsers, VST plugins and Bela board. Glicol's synth-like syntax and powerful audio engine also make it possible to combine high-level synth or sequencer control with low-level sample-accurate audio synthesis, all in real-time.

<!-- Glicol can be used for:
- live coding performance, either in browsers with your friends or in a VST plugin(experimental)
- education of electronic music, DSP and coding
- audio/music app development in browsers, [either CDN or NPM](https://github.com/chaosprint/glicol/tree/main/js)
- Rust audio library, running on Web, Desktop, [DAW](https://github.com/chaosprint/glicol/tree/main/rs/vst), [Bela](https://github.com/chaosprint/glicol/tree/main/rs/bela), etc. -->

## Get started

### 🚀 The Web App
 
The easiest way to try Glicol:

https://glicol.org

> There you can find guides, demos, docs, and apps for collaboration.

<details>
  <summary>Features</summary>
  
  - Near-native, garbage-collection-free and memory-safe real-time audio in web browsers
  
  - Quick reference in consoles with `alt-d`
  
  - The web app automatically loads samples; you can also drag and drop local samples in the browser editor
  
  - Robust error handling: error reported in console, but previous music will continue!
  
  - Mix JavaScript code to create visuals with Hydra synth made by @ojack
  
  - What you see is what you get, i.e. declarative programmering for both code writing and executing: no need to select anything, just change the code and update, Glicol engine will use `LCS` algorithm to handle adding, updating and removing
  
  - Decentralised collaboration using `yjs` and a unique `be-ready` mechanism
</details>

### 🎁 For Audio Dev

|                 |               Description                               |
|-----------------|:---------------------------------------------------------:|
| [NPM Docs](https://glicol.js.org)       | Safe, performant, light-weight and ergonomic audio lib for web apps |
| [Rust Audio Lib](https://github.com/chaosprint/glicol/tree/main/rs/synth)  | Write VST like [this Dattorro reverb plugin](https://github.com/chaosprint/dattorro-vst-rs)  |   
| [Run on Bela](https://github.com/chaosprint/glicol/tree/main/rs/bela) | Run Glicol DSL on Bela board for quick audio prototyping.  |

### 🍿 YouTube Channel
Find Glicol demo vidoes [in this playlist](https://www.youtube.com/playlist?list=PLT4REhRBWaOOrLQxCg5Uw97gEpN-woo1c).

## Philosophy of Glicol

The motivation of Glicol is:

- to help people with zero knowledge of coding and music production to get started with live coding

- to offer experienced music coders a tool for quick prototyping and hacking

In [NIME community](https://nime.org/), it is known as: 
> low entry fee and high ceilings

This is Glicol's philosophy to approach these goals:

- design the language from a new instrument design perspective

- embrace the spirit of the internet for a better experience

Reflected in the implementation:

- Glicol adopts a graph-oriented paradigm

- Glicol can be used in browsers with zero-installation

### Graph-oriented

The basic idea of Glicol is to connect different nodes like synth modules.

All you need to know is the audio input/output behaviour of each node.

Two ways for connecting: `>>` and `~reference`:
```
// amplitude modulation and lazy evaluation example
// chain with ~ is a ref chain and will not be sent to the DAC

o: sin 440 >> mul ~amp
~amp: sin 1.0 >> mul 0.3 >> add 0.5
```
<!-- Sometimes, constraints make it easier to learn and use. -->

It also applies to sequencer and sampler:
```
// sequencer pattern
// first divide one bar with space
// then further divide each part based on midi number and rest(_)

o: speed 2.0 >> seq 60 _~a _ 48__67
>> sp \blip

// quantity alters probability
~a: choose 60 60 0 0 72 72
```

As mentioned above, you can try these examples on:

https://glicol.org

If you want, you can even hear how a `seq` node work:
```
o: speed 2.0 >> seq 60 _72 _ 48__67 >> mul 0.5
```

This is actually analogous to how hardware module pass signals.

It is very easy to remember and to get started.

When Glicol is used in education, we can let students see and hear each node, even including 'envelope'.

Just leave the introduction of data types, `Object` or `Function` later when we mix JavaScript with Glicol.

### Zero-installation

For the audio engine, instead of mapping it to existing audio lib like `SuperCollider`, I decide to do it the hard way:

- write the parser in Rust

- write the audio engine in Rust that works seamlessly with the AST processing

- port it to browsers using `WebAssembly`, `AudioWorklet` and `SharedArrayBuffer`

The main reason is to explore performant audio in browsers for easy access and live coding collaboration.

The reward is that we now have an Rust audio lib called `glicol_synth`:

It can run on Web, Desktop, DAW, Bela board, etc.

And one more thing.

To write everything from low-level also opens the door for `meta` node.

Now I can explain to students, the `hello world` tone can also be written in this way:
```
o: meta `
    output.pad(128, 0.0);
    for i in 0..128 {
        output[i] = sin(2*PI()*phase) ;
        phase += 440.0 / sr;
    };
    while phase > 1.0 { phase -= 1.0 };
    output
`
```

## Roadmap

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
- [x] `0.9.0` redesigned architecture; see the release note
- [x] `0.10.0` run as a VST plugin
- [x] `0.11.0` run on Bela
- [x] `0.12.0` distribute as a `npm` package
- [ ] better music expressions, more variation for `seq` nodes
- [ ] exploring new forms of musical interactions
<!-- - [ ] midi support? used in vst? -->
<!-- - [ ] examples for web audio, vst, bela, etc. -->

> Note that Glicol is still highly experimental, so it can be risky for live performances. 
> The API may also change before version 1.0.0.

Please let me know in [issues](https://github.com/chaosprint/glicol/issues) or [discussions](https://github.com/chaosprint/glicol/discussions):
- your thoughts on the experience of glicol
- new feature suggestion
- bug report, especially the code that causes a `panic` in browser console
- missing and confusion in guides and reference on the website
