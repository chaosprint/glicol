<div align="center">
  <br />
  <p>
    <a href="https://glicol.org"><img src="./logo.png" width="200" /></a>
  </p>
</div>

GLICOL (an acronym for "graph-oriented live coding language") is a computer music language written in Rust.

The project has mainly two parts:
1. The language and audio engine (this repo).
    - It can be used as a standalone Rust audio library.
    - It is also shipped as one single JavaScript package.
2. The web interface (https://glicol.org).
    - You can find interactive guides and docs for Glicol.
    - You can make decentralised collaborative live coding music/audiovisual performance.

## Get started

For quick start, see https://glicol.org.

If you want to know more about the dev part, please continue to read.

## Repo structure

The structure shows that Glicol can be used independently as a JavaScript library in the browser, or used as an audio library for other Rust projects:

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

### Rust

The `rs` folder contains the Rust code for Glicol.

The `rs/main` is the main entrance of crate `glicol`.

The `rs/parser` is the `glicol_parser` crate, which provides the parsing tool for Glicol syntax.

The `rs/synth` is the `glicol_synth` crate, which contains the DSP code for Glicol and can be used as an independent audio lib.

The `rs/macro` provides Rust macros for developing Glicol extensions.

The `rs/ext` is the Glicol extensions, which relies on `glicol_synth`, `glicol_parser` and `glicol_macro`. The idea is to use the essential nodes in `glicol_synth` to form some more complicated nodes, e.g. reverb nodes. Developers can write new node with Glicol syntax in Rust.

The `rs/wasm` is basically the glue code for compiling the `glicol` crate into a WebAssembly file.

See the [README.md](./rs/README.md) file in the `rs` folder for details.

### JavaScript

The `js` folder contains the Glicol distribution for the web platform.

The usage is very easy. Just include this into your `index.html`:

```
<script src="https://cdn.jsdelivr.net/gh/chaosprint/glicol@latest/js/src/glicol.js"></script>
```

See the `README.md` in `js` folder for details.

## Contribution

Suggestions, bug reporting, or PR are warmly welcomed.

## Acknowledgement

This work was partially supported by the Research Council of Norway through its Centres of Excellence scheme, project number 262762 and by NordForsk's Nordic University Hub Nordic Sound and Music Computing Network NordicSMC, project number 86892.

Special thanks to:

- [RustAudio](https://github.com/RustAudio) community, [mitchmindtree](https://github.com/mitchmindtree), and [dasp](https://github.com/RustAudio/dasp) contributors
- [Paul Adenot](https://github.com/padenot) and [ringbuf.js](https://github.com/padenot/ringbuf.js) contributors
- [Kevin Jahns](https://github.com/dmonad), [yjs](https://github.com/yjs) community and [y-codemirror](https://github.com/yjs/y-codemirror) contributors

## License

The MIT License (MIT)

Copyright (c) 2020 - present Qichao Lan (chaosprint)
