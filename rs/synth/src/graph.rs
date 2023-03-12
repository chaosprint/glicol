// The MIT License (MIT)

// Copyright (c) 2016 RustAudio Developers

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::{
    buffer::Buffer,
    node::Input,
    node::Node,
    BoxedNode,
};
use hashbrown::HashMap;
use petgraph::data::{DataMap, DataMapMut};
use petgraph::visit::{
    Data, DfsPostOrder, GraphBase, IntoNeighborsDirected, Reversed, //NodeCount, NodeIndexable, 
    Visitable,
};
use petgraph::{Incoming};

pub struct Processor<G, const N: usize>
where
    G: Visitable,
{
    // State related to the traversal of the audio graph starting from the output node.
    dfs_post_order: DfsPostOrder<G::NodeId, G::Map>,
    // Solely for collecting the inputs of a node in order to apply its `Node::process` method.
    inputs: HashMap<usize, Input<N>>,
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
        let inputs = HashMap::new(); //Vec::with_capacity(max_nodes);
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
            // println!("{:?}", (&*graph).to_index(in_n));

            let input_container = graph.node_weight(in_n).expect(NO_NODE);
            let input = Input::new(&input_container.buffers, (&*graph).to_index(in_n));
            processor.inputs.insert((&*graph).to_index(in_n), input);
        }
        // Here we deference our raw pointer to the `NodeData`. The only references to the graph at
        // this point in time are the input references and the node itself. We know that the input
        // references do not alias our node's mutable reference as we explicitly check for it while
        // looping through the inputs above.
        unsafe {
            (*data)
                .node
                .process(&mut processor.inputs, &mut (*data).buffers);
        }
    }
}