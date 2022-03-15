## Introduction

This folder contains files for the engine of Glicol written in Rust.

The engine is divided into several crates.

This is mainly because that proc macro requires such a structure.

Yet this makes it clear that `glicol_synth` crate can be used as a standalone Rust audio library.

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