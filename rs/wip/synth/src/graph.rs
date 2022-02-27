use crate::{
    Message,
    NodeData, 
    BoxedNodeSend, 
    Processor,
    Node, Buffer,
    node::Sum
};
use petgraph::{
    graph::NodeIndex,
    stable_graph::StableDiGraph,
    prelude::EdgeIndex
};

pub type GlicolNodeData<const N: usize> = NodeData<BoxedNodeSend<N>, N>;
pub type GlicolGraph<const N: usize> = StableDiGraph<GlicolNodeData<N>, (), u32>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;

pub struct StableGraph<const N: usize> {
    graph: GlicolGraph::<N>,
    pub destination: NodeIndex,
    processor: GlicolProcessor<N>
}

impl<const N: usize> StableGraph<N> {
    /// the easist way to create a stable graph
    /// default stereo output
    pub fn new() -> Self {
        let mut graph = GlicolGraph::<N>::with_capacity(1024, 1024);
        let destination = graph.add_node( NodeData::new2( BoxedNodeSend::<N>::new(Sum) ) );
        Self {
            graph,
            destination,
            processor: GlicolProcessor::<N>::with_capacity(1024),
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

    pub fn add_multi_chan_node<T>(&mut self, node: T) -> NodeIndex
    where T: Node<N> + Send + 'static,
    {
        let node_index = self.graph.add_node( // channel?
            NodeData::multi_chan_node (2,
                BoxedNodeSend::<N>::new(
                    node
                )
            )
        );
        return node_index
    }

    pub fn connect(&mut self, from: NodeIndex, to: NodeIndex) -> EdgeIndex {
        let edge_index = self.graph.add_edge(from, to, ());
        return edge_index
    }

    // pub fn chain(&mut self, chain: Vec<NodeIndex>) -> Vec<EdgeIndex> {
        
    // }

    pub fn next_block(&mut self) -> &[Buffer<N>] {
        self.processor.process(&mut self.graph, self.destination);
        &self.graph[self.destination].buffers
    }

    // pub fn send_msg(&mut self, index: usize, msg: Message, secondary: u8) {
        
    // }
}