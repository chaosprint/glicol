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

For interaction, Glicol choose a WYSIWYG (what-you-see-is-what-you-get) paradigm. Under the hood, Glicol has implemented LCS algorithm to dynamically update the graph in real-time. Together with code preprocessing, Glicol provides smooth transition for oscillators and the `mul` node.

You can learn Glicol, find music example and create decentralised collaboration on its web interface:

https://glicol.org

The web interface has the following featues:
1. run Glicol engine at near-native speed, thanks to WebAssembly
2. garbage-collection-free real-time audio in browsers thanks to AudioWorklet, SharedArrayBuffer
3. error handling and command in browser console: e.g. load your own samples
4. mix JS code with Glicol easily: `o: sin {42*10+20}`
5. create visuals with Hydra

### DSP

Glicol can be used for:

1. Web Audio API, Tone.js alternative
2. VST plugins in Rust
3. Programming on Bela board (has POC, but still WIP)

#### JS

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

## Contribution

I am currently working on:
- [ ] make the error handling more robust
- [ ] add new music examples
- [ ] build new features demanded by music making
- [ ] write and organise better docs and guides
- [ ] give examples for using Glicol as vst, bela and tonejs alternative

I would like to hear particuarly in issues or discussion:
- new features suggestion
- bug report
- missing and confusion in any docs and guides