pub use crate::{
    buffer::Buffer,
    node::{Input, Node},
    NodeData,
    BoxedNode,
    BoxedNodeSend,
    Processor,
    Message,
    Sum
};

use petgraph::{
    graph::NodeIndex,
    prelude::EdgeIndex
};

pub struct AudioContextBuilder<const N: usize> {
    sr: usize,
    channels: usize,
    // stablegraph: bool,
    max_nodes: usize,
    max_edges: usize,
}

impl<const N: usize> AudioContextBuilder<N> {
    pub fn new() -> Self {
        Self {
            sr: 44100,
            channels: 2,
            // stablegraph: false,
            max_nodes: 1024,
            max_edges: 1024
        }
    }

    pub fn sr(self, sr: usize) -> Self {
        Self {
            sr, ..self
        }
    }

    pub fn channels(self, channels: usize) -> Self {
        Self {
            channels, ..self
        }
    }

    pub fn max_nodes(self, max_nodes: usize) -> Self {
        Self {
            max_nodes, ..self
        }
    }

    pub fn max_edges(self, max_edges: usize) -> Self {
        Self {
            max_edges, ..self
        }
    }

    pub fn build(self) -> AudioContext<N> {
        AudioContext::new(
            AudioContextConfig{
                sr: self.sr,
                channels: self.channels,
                max_nodes: self.max_nodes,
                max_edges: self.max_edges,
            }
        )
    }
}

pub struct AudioContextConfig {
    pub sr: usize,
    pub channels: usize,
    // pub stablegraph: bool,
    pub max_nodes: usize,
    pub max_edges: usize,
}

impl std::default::Default for AudioContextConfig {
    fn default() -> Self {
        Self {
            sr: 44100,
            channels: 2,
            max_nodes: 1024,
            max_edges: 1024
        }
    }
}

#[macro_export]
macro_rules! audiocontext {
    ($size:expr, {$($para: ident: $data:expr),*}) => {
        (
            AudioContextBuilder::<$size>::new()$(.$para($data))*.build()
        )
    }
}

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
pub type GlicolGraph<const N: usize> = petgraph::graph::Graph<GlicolNodeData<N>, ()>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;

pub struct AudioContext<const N: usize> {
    pub destination: NodeIndex,
    pub graph: GlicolGraph<N>,
    pub processor: GlicolProcessor<N>
}

impl<const N: usize> AudioContext<N> {
    pub fn new(config: AudioContextConfig) -> Self {
        let mut graph = GlicolGraph::<N>::with_capacity(config.max_nodes, config.max_edges);
        let destination = graph.add_node( NodeData::multi_chan_node(config.channels, BoxedNodeSend::<N>::new(Sum) ) );
        Self {
            graph,
            destination,
            processor: GlicolProcessor::<N>::with_capacity(config.max_nodes),
        }
    }

    /// an alternative to new() specify the estimated max node and edge numbers
    /// to avoid dynamic allocation
    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        let mut graph = GlicolGraph::<N>::with_capacity(nodes, edges);
        let destination = graph.add_node( NodeData::new2( BoxedNodeSend::<N>::new( Sum)  ) );
        Self {
            graph,
            destination,
            processor: GlicolProcessor::<N>::with_capacity(nodes),
        }
    }

    pub fn add_mono_node<T>(&mut self, node: T) -> NodeIndex
    where T: Node<N> + Send + 'static,
    {
        let node_index = self.graph.add_node( // channel?
            NodeData::new1(
                BoxedNodeSend::<N>::new(
                    node
                )
            )
        );
        return node_index
    }

    pub fn add_stereo_node<T>(&mut self, node: T) -> NodeIndex
    where T: Node<N> + Send + 'static,
    {
        let node_index = self.graph.add_node( // channel?
            NodeData::new2(
                BoxedNodeSend::<N>::new(
                    node
                )
            )
        );
        return node_index
    }

    pub fn add_multi_chan_node<T>(&mut self, chan: usize, node: T) -> NodeIndex
    where T: Node<N> + Send + 'static,
    {
        let node_index = self.graph.add_node( // channel?
            NodeData::multi_chan_node (chan,
                BoxedNodeSend::<N>::new(
                    node
                )
            )
        );
        return node_index
    }

    pub fn connect(&mut self, from: NodeIndex, to: NodeIndex) -> EdgeIndex {
        let edge_index = self.graph.add_edge(from, to, ());
        self.graph[to].node.send_msg(Message::Index(from.index()));
        return edge_index
    }

    pub fn connect_with_order(&mut self, from: NodeIndex, to: NodeIndex, pos: usize) -> EdgeIndex {
        let edge_index = self.graph.add_edge(from, to, ());
        self.graph[to].node.send_msg(Message::IndexOrder(pos, from.index()));
        return edge_index
    }

    pub fn chain(&mut self, chain: Vec<NodeIndex>) -> Vec<EdgeIndex> {
        let mut v = vec![];
        for pair in chain.windows(2) {
            v.push(self.graph.add_edge(pair[0], pair[1], ()));
            self.graph[pair[1]].node.send_msg(Message::Index(pair[0].index()));
        };
        v
    }

    pub fn add_node_chain(&mut self, chain: Vec<NodeData<BoxedNodeSend<N>, N>>) -> (Vec<NodeIndex>, Vec<EdgeIndex>)
    {
        let mut v = vec![];
        let mut j = vec![];
        for node in chain {
            let id = self.graph.add_node(node);
            v.push(id);
        };
        for pair in v.windows(2) {
            j.push(self.graph.add_edge(pair[0], pair[1], ()));
        };
        (v, j)
    }

    pub fn next_block(&mut self) -> &[Buffer<N>] {
        self.processor.process(&mut self.graph, self.destination);
        &self.graph[self.destination].buffers
    }

    pub fn send_msg(&mut self, index: NodeIndex, msg: Message) {
        self.graph[index].node.send_msg(msg);
    }
}