# Glicol-rs

This folder contains files for the engine of Glicol written in Rust.

The engine is divided into several crates, including `glicol_parser`, `glicol_synth`, `glicol_ext` and `glicol_macro`.

The `glicol_parser` is used for parsing Glicol syntax.

In this process, the `glicol_synth` is used to build the essential audio nodes, etc.

For some extra nodes such as `plate` reverb, `glicol_ext` is used.

`glicol_ext` depends on `glicol_synth`, `glicol_parser` and `glicol-macro`.

# As standalone audio lib

Glicol can be used as an independent audio library for other Rust projects.

There are two ways to use Glicol-rs independently. One is to use the big `glicol` crate, and the other is to only use the essential nodes in `glicol_synth` crate.

Remember:
```
glicol_ext = glicol_synth + glicol_parser + glicol_macro
```
```
glicol = glicol_parser + glicol_synth + glicol_ext
```

In the big `glicol` crate, users can use nodes from `glicol_ext`, but have to write more codes:
```
use dasp_graph::{NodeData, BoxedNodeSend};
use glicol::*;
use glicol_synth::operation::{mul::*, add::*};
use glicol_synth::oscillator::sin_osc::*;

fn main() {
    let mut engine = Engine::new(44100);
    // return a vec of nodeindex
    let i_mod = engine.make_chain(vec![sin_osc!({freq: 10.0}), mul!(0.5), add!(0.5)]);
    let out = engine.make_chain(vec![sin_osc!({freq: 440.0}), mul!()]);
    engine.make_edge(i_mod[2], out[1]);
    engine.process(out[1]); // this is a simplified method for calling processor on graph
    println!("First block {:?}", engine.graph[out[1]].buffers);
}
```

But a more ergonomic way is to use those nodes with the `glicol_macro`:

```
use glicol_macro::*;
use glicol_synth::{SimpleGraph};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;

fn main() {
    let num = 0.1;
    let mut g = make_graph!{
        out: ~input >> add #num;
    };
    println!("{:?}", g.next_block(&mut [0.0; 128]));
}
```