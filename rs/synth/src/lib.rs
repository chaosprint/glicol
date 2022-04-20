#![doc = include_str!("../README.md")]
mod context;
pub use context::*;

mod graph;
pub use graph::*;

mod node;
pub use node::{Input, Node};
pub use node::{
    oscillator, 
    filter, 
    effect, 
    envelope, 
    operator, 
    sequencer, 
    signal,
    delay,
    compound,
    synth
};
// pub use node::*; // TODO: Do not expose every struct here

mod buffer;
pub use buffer::Buffer;

#[cfg(feature = "node-sampling")]
pub use node::{sampling};

#[cfg(feature = "node-dynamic")]
pub use node::{dynamic};

#[cfg(feature = "node-boxed")]
pub use node::{BoxedNode, BoxedNodeSend};

#[cfg(feature = "node-sum")]
pub use node::{Sum, Sum2};

#[cfg(feature = "node-pass")]
pub use node::{Pass};

// #[cfg(feature = "node-pass")]
// pub use node::{Pass};

use hashbrown::HashMap;
// pub use hashbrown::HashMap;
// pub use arrayvec::ArrayVec;

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
    SetToSymbol(u8, String),
    SetToSamples(u8, (&'static [f32], usize, usize)),
    SetSamplePattern(Vec<(String, f32)>, f32, HashMap<String, (&'static [f32], usize, usize)>),
    SetPattern(Vec<(f32, f32)>, f32),
    SetToSeq(u8, Vec::<(f32, GlicolPara)>),
    SetRefOrder(HashMap<String, usize>),
    SetBPM(f32),
    SetSampleRate(usize),
    MainInput(petgraph::graph::NodeIndex),
    SidechainInput(petgraph::graph::NodeIndex),
    Index(usize),
    IndexOrder(usize, usize),
    ResetOrder
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum GlicolPara {
    Number(f32),
    NumberList(Vec<f32>),
    Reference(String),
    SampleSymbol(String), // symbol is for sample only
    Symbol(String),
    Sequence(Vec::<(f32, GlicolPara)>),
    Pattern(Vec::<(GlicolPara, f32)>, f32),
    Event(Vec::<(GlicolPara, f32)>)
}