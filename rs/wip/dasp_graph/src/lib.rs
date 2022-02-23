//! A crate for dynamically creating and editing audio graphs.
//!
//! `dasp_graph` is targeted towards users who require an efficient yet flexible and dynamically
//! configurable audio graph. Use cases might include virtual mixers, digital audio workstations,
//! game audio systems, virtual modular synthesizers and more.
//!
//! # Overview
//!
//! A `dasp` graph is composed of **nodes** and **edges**.
//!
//! Each node contains an instance of a type that implements the [`Node`
//! trait](./node/trait.Node.html). This is normally an audio source (input), processor (effect) or
//! sink (output). The `Node` trait is the core abstraction of `dasp_graph` and allows for trivial
//! re-use of audio nodes between projects and libraries. By implementing `Node` for your audio
//! instruments, effects, generators and processors, they can be easily composed together within a
//! graph and shared with future projects or other `dasp` users. `dasp_graph` provides a suite of
//! popular node implementations out of the box, each of which may be accessed by enabling [their
//! associated features](./index.html#optional-features).
//!
//! The edges of a `dasp` graph are empty and simply describe the direction of audio flow
//! through the graph. That is, the edge *a -> b* describes that the audio output of node *a* will
//! be used as an input to node *b*.
//!
//! Once we have added our nodes and edges describing the flow of audio through our graph, we can
//! repeatedly process and retrieve audio from it using the [`Processor`](./struct.Processor.html)
//! type.
//!
//! # Comparison to `dasp_signal`
//!
//! While [`dasp_signal`](https://docs.rs/dasp_signal) and its [`Signal`
//! trait](https://docs.rs/dasp_signal/latest/dasp_signal/trait.Signal.html) are already well
//! suited towards composing audio graphs, there are certain use cases where they can cause
//! friction. Use cases that require dynamically adding or removing nodes, mapping between
//! dynamically changing channel layouts, or writing the output of one node to multiple others are
//! all difficult to achieve in an elegant manner using `dasp_signal`.
//!
//! `dasp_graph` is designed in a manner that better handles these cases. The flat ownership model
//! where the graph owns all nodes makes it trivial to add or remove nodes and edges at runtime.
//! Nodes can specify the number of buffers that they support during construction, making it easy
//! to handle different channel layouts. Adding multiple outputs to a node (including predecessors
//! to enable cycles) is trivial due to `dasp_graph`'s requirement for a fixed sample rate across
//! the whole graph.
//!
//! On the other hand, `dasp_graph`'s requirement for a fixed sample rate can also be a limitation.
//! A `dasp_graph` cannot be composed of nodes with differing input sample rates meaning it is
//! unsuitable for writing a streaming sample rate converter. `dasp_graph`'s fixed buffer size
//! results in another limitation. It implies that when creating a cycle within the graph, a
//! minimum delay of `Buffer::LEN` is incurred at the edge causing the cycle. This makes it
//! tricky to compose per-sample feedback delays by using cycles in the graph.
//!
//! | Feature                                           | `dasp_graph`  | `dasp_signal` |
//! | ------------------------------------------------- |:-------------:|:-------------:|
//! | Easily dynamically add/remove nodes/edges         | ✓             | ✗             |
//! | Easily write output of node to multiple others    | ✓             | ✗             |
//! | Dynamic channel layout                            | ✓             | ✗             |
//! | Efficiently implement per-sample feedback         | ✗             | ✓             |
//! | Support variable input sample rate per node       | ✗             | ✓             |
//!
//! In general, `dasp_signal` tends to be better suited towards the composition of fixed or static
//! graphs where the number of channels are known ahead of time. It is perfect for small, fixed,
//! static graph structures like a simple standalone synthesizer/sampler or small
//! processors/effects like sample-rate converters or pitch shifters. `dasp_graph` on the other
//! hand is better suited at a higher level where flexibility is a priority, e.g. a virtual mixing
//! console or, the underlying graph for a digital audio workstation or a virtual modular
//! synthesizer.
//!
//! Generally, it is likely that `dasp_signal` will be more useful for writing `Node`
//! implementations for audio sources and effects, while `dasp_graph` will be well suited to
//! dynamically composing these nodes together in a flexible manner.
//!
//! # Graph types
//!
//! Rather than providing a fixed type of graph to work with, `dasp_graph` utilises the `petgraph`
//! traits to expose a generic interface allowing users to select the graph type that bests suits
//! their application or implement their own.
//!
//! **Graph**
//!
//! The [`petgraph::graph::Graph`](https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html)
//! type is a standard graph type exposed by `petgraph`. The type is simply an interface around two
//! `Vec`s, one containing the nodes and one containing the edges.  Adding nodes returns a unique
//! identifier that can be used to index into the graph. As long as the graph is intialised with a
//! sufficient capacity for both `Vec`s, adding nodes while avoiding dynamic allocation is simple.
//!
//! **StableGraph**
//!
//! One significant caveat with the `Graph` type is that removing a node invalidates any existing
//! indices that refer to the following nodes stored within the graph's node `Vec`. The
//! [`petgraph::stable_graph::StableGraph`](https://docs.rs/petgraph/latest/petgraph/stable_graph/struct.StableGraph.html)
//! type avoids this issue by storing each node in and enum.  When a node is "removed", the element
//! simply switches to a variant that indicates its slot is available for use the next time
//! `add_node` is called.
//!
//! In summary, if you require the ability to dynamically remove nodes from your graph you should
//! prefer the `StableGraph` type. Otherwise, the `Graph` type is likely well suited.
//!
//! If neither of these graphs fit your use case, consider implementing the necessary petgraph
//! traits for your own graph type. You can find the necessary traits by checking the trait bounds
//! on the graph argument to the `dasp_graph` functions you intend to use.
//!
//! # Optional Features
//!
//! Each of the provided node implementations are available by default, however these may be
//! disabled by disabling default features. You can then enable only the implementations you
//! require with the following features:
//!
//! - The **node-boxed** feature provides a `Node` implementation for `Box<dyn Node>`. This is
//!   particularly useful for working with a graph composed of many different node types.
//! - The **node-graph** feature provides an implementation of `Node` for a type that encapsulates
//!   another `dasp` graph type. This allows for composing individual nodes from graphs of other
//!   nodes.
//! - The **node-signal** feature provides an implementation of `Node` for `dyn Signal`. This is
//!   useful when designing nodes using `dasp_signal`.
//! - The **node-delay** feature provides a simple multi-channel `Delay` node.
//! - The **node-pass** feature provides a `Pass` node that simply passes audio from its
//!   inputs to its outputs.
//! - The **node-sum** feature provides `Sum` and `SumBuffers` `Node` implementations. These are
//!   useful for mixing together multiple inputs, and for simple mappings between different channel
//!   layouts.
//!
//! ### no_std
//!
//! *TODO: Adding support for `no_std` is pending the addition of support for `no_std` in petgraph.
//! See https://github.com/petgraph/petgraph/pull/238.

pub use buffer::Buffer;
pub use node::{Input, Node};
use petgraph::data::{DataMap, DataMapMut};
use petgraph::visit::{
    Data, DfsPostOrder, GraphBase, IntoNeighborsDirected, NodeCount, NodeIndexable, Reversed,
    Visitable,
};
use petgraph::{Incoming, Outgoing};

#[cfg(feature = "node-boxed")]
pub use node::{BoxedNode, BoxedNodeSend};

mod buffer;
pub mod node;

/// State related to the processing of an audio graph of type `G`.
///
/// The **Processor** allows for the re-use of resources related to traversal and requesting audio
/// from the graph. This makes it easier to avoid dynamic allocation within a high-priority audio
/// context.
///
/// # Example
///
/// ```
/// use dasp_graph::{Node, NodeData};
/// # use dasp_graph::{Buffer, Input};
/// use petgraph;
/// #
/// # // The node type. (Hint: Use existing node impls by enabling their associated features).
/// # struct MyNode<const N: usize>;
///
/// // Chose a type of graph for audio processing.
/// type Graph = petgraph::graph::DiGraph<NodeData<MyNode::<128>, 128>, (), u32>;
/// // Create a short-hand for our processor type.
/// type Processor = dasp_graph::Processor<Graph, 128>;
/// #
/// # impl<const N: usize> Node<N> for MyNode<N> {
/// #     // ...
/// #    fn process(&mut self, _inputs: &[Input<N>], _output: &mut [Buffer<N>]) {
/// #    }
/// # }
///
/// fn main() {
///     // Create a graph and a processor with some suitable capacity to avoid dynamic allocation.
///     let max_nodes = 1024;
///     let max_edges = 1024;
///     let mut g = Graph::with_capacity(max_nodes, max_edges);
///     let mut p = Processor::with_capacity(max_nodes);
///
///     // Add some nodes and edges...
/// #    let n_id = g.add_node(NodeData::new1(MyNode::<128>));
///
///     // Process all nodes within the graph that output to the node at `n_id`.
///     p.process(&mut g, n_id);
/// }
/// ```
pub struct Processor<G, const N: usize>
where
    G: Visitable,
{
    // State related to the traversal of the audio graph starting from the output node.
    dfs_post_order: DfsPostOrder<G::NodeId, G::Map>,
    // Solely for collecting the inputs of a node in order to apply its `Node::process` method.
    inputs: Vec<node::Input<N>>,
}

/// For use as the node weight within a dasp graph. Contains the node and its buffers.
///
/// For a graph to be compatible with a graph **Processor**, its node weights must be of type
/// `NodeData<T>`, where `T` is some type that implements the `Node` trait.
pub struct NodeData<T: ?Sized, const N: usize> {
    /// The buffers to which the `node` writes audio data during a call to its `process` method.
    ///
    /// Generally, each buffer stored within `buffers` corresponds to a unique audio channel. E.g.
    /// a node processing mono data would store one buffer, a node processing stereo data would
    /// store two, and so on.
    pub buffers: Vec<Buffer<N>>,
    pub node: T,
}

impl<G, const N: usize> Processor<G, N>
where
    G: Visitable,
{
    /// Construct a new graph processor from the given maximum anticipated node count.
    ///
    /// As long as this node count is not exceeded, the **Processor** should never require dynamic
    /// allocation following construction.
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

    /// Process audio through the subgraph ending at the node with the given ID.
    ///
    /// Specifically, this traverses nodes in depth-first-search *post* order where the edges of
    /// the graph are reversed. This is equivalent to the topological order of all nodes that are
    /// connected to the inputs of the given `node`. This ensures that all inputs of each node are
    /// visited before the node itself.
    ///
    /// The `Node::process` method is called on each node as they are visited in the traversal.
    ///
    /// Upon returning, the buffers of each visited node will contain the audio processed by their
    /// respective nodes.
    ///
    /// Supports all graphs that implement the necessary petgraph traits and whose nodes are of
    /// type `NodeData<T>` where `T` implements the `Node` trait.
    ///
    /// **Panics** if there is no node for the given index.
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

/// Process audio through the subgraph ending at the node with the given ID.
///
/// Specifically, this traverses nodes in depth-first-search *post* order where the edges of
/// the graph are reversed. This is equivalent to the topological order of all nodes that are
/// connected to the inputs of the given `node`. This ensures that all inputs of each node are
/// visited before the node itself.
///
/// The `Node::process` method is called on each node as they are visited in the traversal.
///
/// Upon returning, the buffers of each visited node will contain the audio processed by their
/// respective nodes.
///
/// Supports all graphs that implement the necessary petgraph traits and whose nodes are of
/// type `NodeData<T>` where `T` implements the `Node` trait.
///
/// **Panics** if there is no node for the given index.
pub fn process<G, T, const N: usize>(
    processor: &mut Processor<G, N>,
    graph: &mut G,
    node: G::NodeId,
) where
    G: Data<NodeWeight = NodeData<T, N>> + DataMapMut + Visitable,
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
            let input_container = graph.node_weight(in_n).expect(NO_NODE);
            let input = node::Input::new(&input_container.buffers);
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
