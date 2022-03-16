## Introduction

This folder contains files for the engine of Glicol written in Rust.

The `main` provides an `Engine` that takes the code as input, stores samples, and yield `next_block` of audio constantly.

The `parser` offers the `get_ast` function for the `Engine` to parse the code.

The `macros` is basically some proc macros to make some dev work easier.

The `wasm` crate exports the `Engine` to a WebAssembly file which can be used in web browsers.

The `synth` provides all the audio support.

## Glicol synth as a standalone Rust audio library

You can write an audio project like this:

```
use glicol_synth::{AudioContextBuilder, signal::ConstSig, Message};

fn main() {
    let mut context = AudioContextBuilder::<128>::new()
    .sr(44100).channels(1).build();

    let node_a = context.add_mono_node(ConstSig::new(42.));
    context.connect(node_a, context.destination);
    println!("first block {:?}", context.next_block());

    context.send_msg(node_a, Message::SetToNumber((0, 100.)) );
    println!("second block, after msg {:?}", context.next_block());
}
```

See [./synth](./synth) folder for more details.

## Try it out

First you should have Rust compiler installed. Make sure you can call `cargo` in your terminal.

Then:
```
git clone https://github.com/chaosprint/glicol.git
cd glicol/rs/synth
cargo run --example hello
cargo run --example chain
```

You can explore more examples in the `./rs/synth/` folder.

## License

The MIT License (MIT)

Copyright (c) 2020 - present Qichao Lan (chaosprint)