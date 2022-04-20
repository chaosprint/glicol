## What's this?

This is a light-weight, garbage-collection free, memory-safe and easy-to-use audio library for browsers. It's written in Rust and ported to JS via WebAssembly and runs in AudioWorklet. The communication is realised with SharedArrayBuffer*.

> *Without SAB, you can still use Glicol. However, to get the best audio performance, you need to have `cross-origin isolation` enabled on the web server (both the dev server and the one you deploy your web app) to use this package. For `vite` dev server, you can use my plugin [here](https://github.com/chaosprint/vite-plugin-cross-origin-isolation). For deployment on `Netlify` or `Firebase`, check their docs for editing the header files. If you use a customised server, you have to figure it out yourself.

## Why `glicol.js`?

### Light-weight
`glicol.js` is only 2.1 MB whereas `tone.js` is 11.3 MB. This means that you can save more bandwidth and increase the loading speed for your web app.

> It's possible to make it even smaller in the future by using only a module of Glicol.

### Garbage-collection free and memory-safe

Glicol audio engine is written in Rust, a system-level programming language that is widely seen as the modern alternative to C/C++ and is used in many scenarios that require the top-level performance.

For an audio library, to be GC-free is very meaningful. And to guarantee the memory-safety is even more important for many musical contexts such as live performances.

> A negative example is memory leaking, which will look fine at the first glance, but when as the time goes by, say 20 minutes, the memory issue will ruin the whole thing.

Glicol audio engine has been proved to be working in live coding music performances in browsers, e.g.

https://youtu.be/atoTujbQdwI

Rust is also famous for its error handling. `glicol.js` has taken advantage of that and offers a robust error handling mechanism. The principle is "Musique Non-Stop", i.e. when there is an error, it will be reported in the console while the music will continue as before.

### Easy to use

With the top-level audio performance in the browser, Glicol is yet easy to use. The balance between minimalism and readability/ergonomics is consistent in the API designing. After you `npm i glicol`, you can just write:

```js
import Glicol from "glicol"
const glicol = new Glicol()
```

Then write the graph in this way:

```js
glicol.play({
    "o": sin(440).mul("~am"),
    "~am": sin(0.2).mul(0.3).add(0.5)
})
```

Simple as that.

No need to create a node, and then connect it everywhere.

Note that there are two `chains` here, one is called `o` and the other is `~am`.

Only the chains without a `~` in their names will be sent to the output destination.

Therefore, it's very intuitive to seperate the modulator, although everything is working at audio rate*.

> *Glicol is an audio-first domain specific language.

Wanna some change/update?

Just call:

```js
glicol.play({
    "o": sin(110).mul("~am"),
    "~am": sin(0.2).mul(0.3).add(0.5)
})
```

The engine will analyse the difference and only update those nodes modified. :)

Yet a lighter way to do it is to write:

```js
// chain "o", node_index 0, param 0, set to 110
glicol.sendMsg(`o, 0, 0, 110`)
```

## API reference (coming soon...)

> As this is not a stable version yet, the APIs may significantly change in the future. If you wish to test or use it in a project, please contact me.

## Alternative usage - Glicol DSL

Under the hood, the JS style graph will be converted to Glicol's syntax for sending to the engine.

To learn more about this syntax, see:

https://glicol.org

After you are familiar with the syntax, you can write your audio graph logic like this:

```js
glicol.run(`o: saw 50 >> lpf 300.0 1.0`)
```

You can call the `run` again if you want to change some parameters. There won't be an update to the whole graph. Instead, the Glicol Rust engine will be smart enough to tell the difference and only update where is modified.

```js
glicol.run(`o: saw 50 >> lpf 300.0 1.0`)
```

Another way is to send message to the engine:

```js
// chain "o", node_index 0, param 0, set to 110
glicol.sendMsg(`o, 0, 0, 110`);
```

You can use it with GUI, see this example:

https://glicol-npm.netlify.app

Multiple message in one String is also possible.

```js
glicol.sendMsg(`o, 0, 0, 110; o, 1, 0, 500; o, 1, 1, 0.8`);
```

## Extension

You can provide an `audioContext` to glicol and use the output of glicol to another node from that `audioContext`:

```js
import Glicol from 'glicol'

const myAudioContext = new AudioContext()
const gainNode = myAudioContext.createGain();
gainNode.gain.value = 0.1
gainNode.connect(myAudioContext.destination)

const glicol = new Glicol({
    audioContext: myAudioContext,
    connectTo: gainNode
})
```

## Feedback

There are many todos for this package. Please let me know your thoughts and suggestions here:

https://github.com/chaosprint/glicol

`Issues` or `Discussion` are both fine.

## Dev note (not for users)
```
sudo pnpm link --dir /usr/local/lib/ glicol
```