//! `glicol_synth` is the audio engine of glicol computer music language.
//! it can be used as a standalone audio library.
//! the api is quite intuitive:
//! 
//! ```
//! use glicol_synth::{AudioContextBuilder, signal::ConstSig, Message};

//! fn main() {
//!    let mut context = AudioContextBuilder::<128>::new()
//!    .sr(44100).channels(1).build();
//!
//!    let node_a = context.add_mono_node(ConstSig::new(42.));
//!    context.connect(node_a, context.destination);
//!    println!("first block {:?}", context.next_block());
//!
//!    context.send_msg(node_a, Message::SetToNumber((0, 100.)) );
//!    println!("second block, after msg {:?}", context.next_block());
//! }
//! ```
//! 
//! ## Overview
//! `glicol_synth` begins with a fork of the dasp_graph crate, written by mitchmindtree.
//! many features and contents are added:
//! - use const generics for a customisable buffer size
//! - replace the input from vec to a map, so users can use a node id to select input
//! - users can send message to each node in real-time for interaction
//! - add a higher level audiocontext for easier APIs
//! - many useful audio nodes from oscillators, filters, etc.
mod context;
pub use context::*;

mod graph;
pub use graph::*;

mod node;
pub use node::{Input, Node};
pub use node::*;

mod buffer;
pub use buffer::Buffer;

#[cfg(feature = "node-boxed")]
pub use node::{BoxedNode, BoxedNodeSend};

#[cfg(feature = "node-sum")]
pub use node::{Sum};

#[cfg(feature = "node-pass")]
pub use node::{Pass};

pub use hashbrown::HashMap;
pub use arrayvec::ArrayVec;

#[macro_export]
macro_rules! impl_to_boxed_nodedata {
    () => {
        pub fn to_boxed_nodedata<const N: usize>(self, channels: usize) -> NodeData<BoxedNodeSend<N>, N> {
            NodeData::multi_chan_node(channels, BoxedNodeSend::<N>::new( self ) )
        }
    };
}

#[derive(Debug, Clone)]
pub enum Message {
    SetToNumber(u8, f32),
    SetToNumberList(u8, Vec<f32>),
    SetToSymbol(u8, &'static str),
    SetToSamples(u8, (&'static [f32], usize)),
    SetToSeq(u8, Vec::<(f32, GlicolPara<'static>)>),
    SetRefOrder(HashMap<&'static str, usize>),
    SetBPM(f32),
    SetSampleRate(usize),
    MainInput(petgraph::graph::NodeIndex),
    SidechainInput(petgraph::graph::NodeIndex),
    Index(usize),
    IndexOrder(usize, usize),
    ResetOrder
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum GlicolPara<'a> {
    Number(f32),
    NumberList(Vec<f32>),
    Reference(&'a str),
    // RefList(Vec<&'a str>),
    SampleSymbol(&'a str), // symbol is for sample only
    Symbol(&'a str),
    Sequence(Vec::<(f32, GlicolPara<'a>)>),
}
