pub use buffer::Buffer;
pub use node::{Input, Node};
use petgraph::data::{DataMap, DataMapMut};
use petgraph::visit::{
    Data, DfsPostOrder, GraphBase, IntoNeighborsDirected, NodeCount, NodeIndexable, Reversed,
    Visitable,
};
use petgraph::{Incoming, Outgoing};
use petgraph::{
    graph::NodeIndex,
    stable_graph::StableDiGraph,
    graph::DiGraph,
    prelude::EdgeIndex
};

#[cfg(feature = "node-boxed")]
pub use node::{BoxedNode, BoxedNodeSend};

#[cfg(feature = "node-sum")]
pub use node::{Sum};

mod buffer;
pub mod node;

pub mod graph;
pub use graph::StableGraph;

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

    // pub fn stablegraph(self, stablegraph: bool) -> Self {
    //     Self {
    //         stablegraph, ..self
    //     }
    // }

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
            // stablegraph: false,
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
pub type GlicolGraph<const N: usize> = StableDiGraph<GlicolNodeData<N>, (), u32>;
pub type GlicolProcessor<const N: usize> = Processor<GlicolGraph<N>, N>;

pub struct AudioContext<const N: usize> {
    pub destination: NodeIndex,
    graph: GlicolGraph<N>,
    processor: GlicolProcessor<N>
}

impl<const N: usize> AudioContext<N> {
    /// the easist way to create a stable graph
    /// default stereo output
    pub fn new(config: AudioContextConfig) -> Self {
        // let mut graph = match config.stablegraph {
        //     true => StableDiGraph::<GlicolNodeData<N>, (), u32>::with_capacity(config.max_nodes, config.max_edges),
        //     false => DiGraph::<GlicolNodeData<N>, (), u32>::with_capacity(config.max_nodes, config.max_edges),
        // };
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

pub struct Processor<G, const N: usize>
where
    G: Visitable,
{
    // State related to the traversal of the audio graph starting from the output node.
    dfs_post_order: DfsPostOrder<G::NodeId, G::Map>,
    // Solely for collecting the inputs of a node in order to apply its `Node::process` method.
    inputs: Vec<node::Input<N>>,
    // pub processed: Vec<G::NodeId>
}

/// For use as the node weight within a dasp graph. Contains the node and its buffers.
///
/// For a graph to be compatible with a graph **Processor**, its node weights must be of type
/// `NodeData<T>`, where `T` is some type that implements the `Node` trait.
pub struct NodeData<T: ?Sized, const N: usize> {
    pub buffers: Vec<Buffer<N>>,
    pub node: T,
}

impl<G, const N: usize> Processor<G, N>
where
    G: Visitable + petgraph::visit::NodeIndexable,
{
    pub fn with_capacity(max_nodes: usize) -> Self
    where
        G::Map: Default,
    {
        let mut dfs_post_order = DfsPostOrder::default();
        dfs_post_order.stack = Vec::with_capacity(max_nodes);
        let inputs = Vec::with_capacity(max_nodes);
        Self {
            dfs_post_order,
            inputs,
        }
    }
    pub fn process<T>(&mut self, graph: &mut G, node: G::NodeId)
    where
        G: Data<NodeWeight = NodeData<T, N>> + DataMapMut,
        for<'a> &'a G: GraphBase<NodeId = G::NodeId> + IntoNeighborsDirected,
        T: Node<N>,
    {
        process(self, graph, node)
    }
}

impl<T, const N: usize> NodeData<T, N> {
    /// Construct a new **NodeData** from an instance of its node type and buffers.
    pub fn new(node: T, buffers: Vec<Buffer<N>>) -> Self {
        NodeData { node, buffers }
    }

    /// Creates a new **NodeData** with a single buffer.
    pub fn new1(node: T) -> Self {
        Self::new(node, vec![Buffer::SILENT])
    }

    /// Creates a new **NodeData** with two buffers.
    pub fn new2(node: T) -> Self {
        Self::new(node, vec![Buffer::SILENT; 2])
    }

    /// Creates a new **NodeData** with 8 buffers.
    pub fn multi_chan_node(chan: usize, node: T) -> Self {
        Self::new(node, vec![Buffer::SILENT; chan])
    }
}

#[cfg(feature = "node-boxed")]
impl<const N: usize> NodeData<BoxedNode<N>, N> {
    /// The same as **new**, but boxes the given node data before storing it.
    pub fn boxed<T>(node: T, buffers: Vec<Buffer<N>>) -> Self
    where
        T: 'static + Node<N>,
    {
        NodeData::new(BoxedNode(Box::new(node)), buffers)
    }

    /// The same as **new1**, but boxes the given node data before storing it.
    pub fn boxed1<T>(node: T) -> Self
    where
        T: 'static + Node<N>,
    {
        Self::boxed(node, vec![Buffer::SILENT])
    }

    /// The same as **new2**, but boxes the given node data before storing it.
    pub fn boxed2<T>(node: T) -> Self
    where
        T: 'static + Node<N>,
    {
        Self::boxed(node, vec![Buffer::SILENT, Buffer::SILENT])
    }
}

pub fn process<G, T, const N: usize>(
    processor: &mut Processor<G, N>,
    graph: &mut G,
    node: G::NodeId,
) where
    G: Data<NodeWeight = NodeData<T, N>> + DataMapMut + Visitable + petgraph::visit::NodeIndexable,
    for<'a> &'a G: GraphBase<NodeId = G::NodeId> + IntoNeighborsDirected,
    T: Node<N>,
{
    const NO_NODE: &str = "no node exists for the given index";
    processor.dfs_post_order.reset(Reversed(&*graph));
    processor.dfs_post_order.move_to(node);
    while let Some(n) = processor.dfs_post_order.next(Reversed(&*graph)) {
        let data: *mut NodeData<T, N> = graph.node_weight_mut(n).expect(NO_NODE) as *mut _;
        processor.inputs.clear();
        for in_n in (&*graph).neighbors_directed(n, Incoming) {
            // Skip edges that connect the node to itself to avoid aliasing `node`.
            if n == in_n {
                continue;
            }
            println!("{:?}", (&*graph).to_index(in_n));

            let input_container = graph.node_weight(in_n).expect(NO_NODE);
            let input = node::Input::new(&input_container.buffers, (&*graph).to_index(in_n));
            processor.inputs.push(input);
        }
        // Here we deference our raw pointer to the `NodeData`. The only references to the graph at
        // this point in time are the input references and the node itself. We know that the input
        // references do not alias our node's mutable reference as we explicitly check for it while
        // looping through the inputs above.
        unsafe {
            (*data)
                .node
                .process(&processor.inputs, &mut (*data).buffers);
        }
    }
}

/// Produce an iterator yielding IDs for all **source** nodes within the graph.
///
/// A node is considered to be a source node if it has no incoming edges.
pub fn sources<'a, G>(g: &'a G) -> impl 'a + Iterator<Item = G::NodeId>
where
    G: IntoNeighborsDirected + NodeCount + NodeIndexable,
{
    (0..g.node_count())
        .map(move |ix| g.from_index(ix))
        .filter_map(move |id| match g.neighbors_directed(id, Incoming).next() {
            None => Some(id),
            _ => None,
        })
}

/// Produce an iterator yielding IDs for all **sink** nodes within the graph.
///
/// A node is considered to be a **sink** node if it has no outgoing edges.
pub fn sinks<'a, G>(g: &'a G) -> impl 'a + Iterator<Item = G::NodeId>
where
    G: IntoNeighborsDirected + NodeCount + NodeIndexable,
{
    (0..g.node_count())
        .map(move |ix| g.from_index(ix))
        .filter_map(move |id| match g.neighbors_directed(id, Outgoing).next() {
            None => Some(id),
            _ => None,
        })
}

pub enum Message {
    Float(f32),
    MainInput(usize),
    Sidechain(usize),
}