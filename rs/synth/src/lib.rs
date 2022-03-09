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


#[derive(Debug)]
pub enum Message {
    SetToNumber(u8, f32),
    SetToSymbol(u8, &'static str),
    SetToSamples(u8, (&'static [f32], usize)),
    MainInput(petgraph::graph::NodeIndex),
    SidechainInput(petgraph::graph::NodeIndex),
    Index(usize),
    IndexOrder(usize, usize),
    ResetOrder
}