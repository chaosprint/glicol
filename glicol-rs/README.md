# Glicol.rs

This folder contains files for the engine of Glicol written in Rust.

The engine is divided into several crates, including `glicol-lang`, `glicol-synth`, `glicol-ext` and `glicol-macro`.

The `glicol-lang` crate converts user code string into an audio graph.

In this process, the `glicol-synth` is used to construct the essential audio nodes, etc.

For some extra nodes such as `plate` reverb, `glicol-ext` is used.

`glicol-ext` depends on `glicol-synth` and `glicol-macro`.