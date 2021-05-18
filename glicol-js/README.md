# Introduction

A JavaScript library for live coding.

The language and audio engines of Glicol are all wrapped in `glicol.js`.

# Demo

The `index.html` shows how you can use this JavaScript library.

To run the demo:

```
cd glicol-js
npm i
npm run dev
```

# Usage

As shown in the demo, just include this into your html file:
```
<script src="https://cdn.jsdelivr.net/gh/chaosprint/glicol/glicol-js/src/glicol.js"></script>
```

The `run()` function is bind to the window.

You can map it to buttons on the page or even do live coding in the browser console.

Call it for the first time will run the code:
```run(`hello: sin 440`)```

Glicol engine knows you are updating the code if you call the func again.

Call `stop()` function will restart the engine.

# License

The MIT License (MIT)

Copyright (c) 2020 - present Qichao Lan (chaosprint)