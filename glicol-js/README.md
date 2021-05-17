# Introduction

This is the independent version of glicol.js.

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

The `runGlicolCode()` function is bind to the window.

Call it for the first time will run the code:
`runGlicolCode(yourCode)`

Glicol engine will know you are updating the code if you call the func again.

Call `stop()` function will restart the engine.

# License

The MIT License (MIT)

Copyright (c) 2020 - present Qichao Lan (chaosprint)