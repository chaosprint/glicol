## Introduction

A language and DSP engine for live coding.

Glicol is a graph-oriented live coding language.

It has both its language and DSP engine written in Rust.

When compiled to WebAssembly, it can be used in browsers.

## Example

To run the demo:

```
npm i
npm run dev
```

## Usage

Just include this into your `index.html`:

```
<script src="https://cdn.jsdelivr.net/gh/chaosprint/glicol@v0.9.2/js/src/glicol.js"></script>
```

The `run()` function is bind to the window.

You can map it to buttons on the page or even do live coding in the browser console.

Call it for the first time will run the code:
```run(`hello: sin 440`)```

Glicol engine knows you are updating the code if you call the func again.

Call `stop()` function will restart the engine.

> Note that you need to have `cross-origin isolation` enabled on the web server to use Glicol. For vite dev server, you can use my plugin [here](https://github.com/chaosprint/vite-plugin-cross-origin-isolation). For deployment on Netlify or Firebase, check their docs for editing the header files.

## License

The MIT License (MIT)

Copyright (c) 2020 - present Qichao Lan (chaosprint)