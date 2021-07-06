//! Implementation of `Node` for a graph of nodes.
//!
//! Allows for nesting subgraphs within nodes of a graph.

use crate::{Buffer, Input, Node, NodeData, Processor};
use core::marker::PhantomData;
use petgraph::data::DataMapMut;
use petgraph::visit::{Data, GraphBase, IntoNeighborsDirected, Visitable};

pub struct GraphNode<G, T, const N: usize>
where
    G: Visitable,
{
    pub processor: Processor<G, N>,
    pub graph: G,
    pub input_nodes: Vec<G::NodeId>,
    pub output_node: G::NodeId,
    pub node_type: PhantomData<T>,
}

impl<G, T, const N: usize> Node<N> for GraphNode<G, T, N>
where
    G: Data<NodeWeight = NodeData<T, N>> + DataMapMut + Visitable,
    for<'a> &'a G: GraphBase<NodeId = G::NodeId> + IntoNeighborsDirected,
    T: Node<N>,
{
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        let GraphNode {
            ref mut processor,
            ref mut graph,
            ref input_nodes,
            output_node,
            ..
        } = *self;

        // Write the input buffers to the input nodes.
        for (input, &in_n) in inputs.iter().zip(input_nodes) {
            let in_node_bufs = &mut graph
                .node_weight_mut(in_n)
                .expect("no node for graph node's input node ID")
                .buffers;
            for (in_node_buf, in_buf) in in_node_bufs.iter_mut().zip(input.buffers()) {
                in_node_buf.copy_from_slice(in_buf);
            }
        }

        // Process the graph.
        processor.process(graph, output_node);

        // Write the output node buffers to the output buffers.
        let out_node_bufs = &mut graph
            .node_weight_mut(output_node)
            .expect("no node for graph node's output node ID")
            .buffers;
        for (out_buf, out_node_buf) in output.iter_mut().zip(out_node_bufs) {
            out_buf.copy_from_slice(out_node_buf);
        }
    }
}
