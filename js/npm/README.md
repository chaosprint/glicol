## What's this?

This is a light-weight, garbage-collection free, memory-safe and easy-to-use audio library for browsers. It's written in Rust and ported to JS via WebAssembly and runs in AudioWorklet. The communication is realised with SharedArrayBuffer.

> Note that you need to have `cross-origin isolation` enabled on the web server (both the dev server and the one you deploy your web app) to use this package. For `vite` dev server, you can use my plugin [here](https://github.com/chaosprint/vite-plugin-cross-origin-isolation). For deployment on `Netlify` or `Firebase`, check their docs for editing the header files. If you use a customised server, you have to figure it out yourself.

## Usage

This is a proof of concept and the API may change. But after you `npm i glicol`, you can just write:

```js
import Glicol from "glicol"
const glicol = new Glicol()
```

Then you run your main audio graph logic:

```js
glicol.run(`o: saw 50 >> lpf 300.0 1.0`)
```

You can call the `run` again if you want to change some parameters. There won't be an update to the whole graph. Instead, the Glicol Rust engine will be smart enough to tell the difference and only update where is modified.

```js
glicol.run(`o: saw 50 >> lpf 300.0 1.0`)
```

Another style is to send message to the engine:

```js
// track "o", node_index 0, param 0, set to 110
glicol.send_msg(`o, 0, 0, 110`);
```

You can use it with GUI, see this example:

https://glicol-npm.netlify.app

Multiple message in one String is also possible.

```js
glicol.send_msg(`o, 0, 0, 110; o, 1, 0, 500; o, 1, 1, 0.8`);
```

## Usage - JS style

You can also write the graph in this way:

```js
glicol.play({
    "o": sin(440).mul("~am~")
    "~am": sin(0.2).mul(0.3).add(0.5)
})
```

And send message as before:

```js
glicol.send_msg(`o, 0, 0, 110`)
```

## Feedback

There are many todos for this package. Please let me know your thoughts and suggestions here:

https://github.com/chaosprint/glicol

`Issues` or `Discussion` are both fine.

## Dev note (not for users)
```
sudo pnpm link --dir /usr/local/lib/ glicol
```