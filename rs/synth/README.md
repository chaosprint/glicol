# glicol_synth: a graph-based audio DSP library written in Rust

`glicol_synth` is the audio engine of glicol computer music language.
It can be used as a standalone audio library, with quite intuitive APIs:

```rust
use glicol_synth::{AudioContextBuilder, signal::ConstSig, Message};

fn main() {
let mut context = AudioContextBuilder::<16>::new()
.sr(44100).channels(1).build();

let node_a = context.add_mono_node(ConstSig::new(42.));
context.connect(node_a, context.destination);
println!("first block {:?}", context.next_block());

context.send_msg(node_a, Message::SetToNumber(0, 100.) );
println!("second block, after msg {:?}", context.next_block());
}
```

## Overview
`glicol_synth` begins with a fork of the `dasp_graph` crate, written by @mitchmindtree.
many features and contents are added:
- use const generics for a customisable buffer size
- replace the input from vec to a map, so users can use a node id to select input
- users can send message to each node in real-time for interaction
- add a higher level audiocontext for easier APIs
- many useful audio nodes from oscillators, filters, etc.

See the [examples on GitHub](https://github.com/chaosprint/glicol/tree/main/rs/synth/examples) for the basic usage.