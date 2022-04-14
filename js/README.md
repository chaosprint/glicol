## What's this?

This folder contains the JavaScript bindings for [Glicol](https://glicol.org) language and audio engine.

So, you can now use Glicol as the audio engien for your own browser-based music app.

There are two usages: `NPM` or `CDN`.

> Note that you need to have `cross-origin isolation` enabled on the web server to use Glicol. For vite dev server, you can use my plugin [here](https://github.com/chaosprint/vite-plugin-cross-origin-isolation). For deployment on Netlify or Firebase, check their docs for editing the header files. If you use a customised server, you have to figure it out yourself.

## Usage - NPM

### Step 1

Install:
```
npm i glicol
```

Then in your web app:
```js
import Glicol from "glicol"
const glicol = new Glicol()
glicol.run(`o: saw 50 >> lpf 300.0 1.0`)
glicol.send_msg(`o, 0, 0, 110; o, 1, 0, 500; o, 1, 1, 0.8`);
```

You can bind it with the GUI and here's an example:

https://github.com/glicol/glicol-npm-example

## Usage - CDN

This mode exposes all the methods such as `run` or `stop` to the `window` Object.

Just include the following line into your `index.html`:

```
<script src="https://cdn.jsdelivr.net/gh/chaosprint/glicol@latest/js/src/glicol.js"></script>
```

The `run()` function is bind to the window.

You can map it to buttons on the page or even do live coding in the browser console.

Call it for the first time will run the code:
```run(`hello: sin 440`)```

Glicol engine knows you are updating the code if you call the func again.

Call `stop()` function will restart the engine.

To run the demo in this folder:
```
npm i
npm run dev
```

## License

The MIT License (MIT)

Copyright (c) 2020 - present Qichao Lan (chaosprint)