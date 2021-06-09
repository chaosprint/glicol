# Overview

Glicol has its a language engine and an audio engine.

The audio engine is designed to be used both with the language engine or as a standalone library.

This is similar to the client-server architecture on SuperCollider.

# The audio engine

The audio engine is based on the graph and different nodes.

The graph relies on `dasp_graph` crate, which provides the essential data structure and trait.

What I have developed is the DSP code in dirrent nodes and APIs.

Concretely, one can use Glicol audio engine like this:

```
use glicol::*;
use glicol::node::operation::{mul::*, add::*};
use glicol::node::oscillator::sin_osc::*;
// amplitude_modulation
fn main() {
    let mut engine = Engine::new(44100);
    let i_mod = engine.make_chain(vec![sin_osc!({freq: 10.0}), mul!(0.5), add!(0.5)]);
    let out = engine.make_chain(vec![sin_osc!({freq: 440.0}), mul!()]);
    engine.make_edge(i_mod[2], out[1]);
    engine.process(out[1]); // this is a simplified method for calling processor on graph
    println!("First block {:?}", engine.graph[out[1]].buffers);
}
```

# The dummy clock

# Code preprocess

# Ndef