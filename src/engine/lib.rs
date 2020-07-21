//! ## Oscen - a library to build modular software synthesizers.
//! 
//! ### The Oscen architecture is designed with the following objcectives in mind:
//! - **Extensible** — Users of Oscen should be able to create their own synth
//! modules without having to modify any of the library code, e.g. add cases to
//! an enum. This is accomplished by defining the [`Signal`] trait and using trait
//! objects to represent synth modules which can be added to a `Rack` (graph).
//! Signal objects can be downcast using the `Any` trait to access specific
//! features of that particular `SynthModule`.
//!
//! - **Dynamic** - The [`Rack`] should be able to be "patched" while the synth
//! is running, similar to a modular hardware synth. A [`Rack`] is basically a
//! graph where nodes are synth modules, e.g. oscillators, envelope generators
//! and filters. Each node can have many inputs and a single output. Edges 
//! connect the ouput of one node to one of the inputs of another. Since we
//! cannot know the names of the fields of a node (because it's a trait object)
//! We use the `Index` and `IndexMut` traits to access the fields by a `&str`.
//!
//! - **Strongly Typed** - As much as possible have the rust catch errors
//! in our synth at comple time. This is difficult to do in light of the 
//! previous objective and some compromises have to be made. E.g., it is not
//! possible to know at compile time about a patch that will be added while the
//! synth is running.
//!
//! [`Signal`]: signal/trait.Signal.html
//! [`Rack`]: signal/struct.Rack.html


/// A collection of some basic audio filters.
pub mod filters;
/// An implementation of *freeverb*.
pub mod reverb;
/// Envelope generators.
pub mod envelopes;
/// Core Oscen types and traits.
pub mod signal;
/// Syth modules for combining other sytn modules.
pub mod operators;
/// Some common (and some less common) oscillators.
pub mod oscillators;
/// Wave shaping.
pub mod shaping;
/// Midi interface nodes.
pub mod midi;
/// Utilites.
pub mod utils;
/// Instruments.
pub mod instruments;
/// Sequencer
pub mod sequencer;
