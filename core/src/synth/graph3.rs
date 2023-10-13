use crate::{Mix, Mul, Node, SinOsc};
use anyhow::Context;
// use hashbrown::HashMap;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::Walker;
use petgraph::{prelude::*, visit::Reversed};
// use slotmap::DefaultKey;
// use smallvec::SmallVec;

#[derive(Debug)]
pub struct Graph3<const N: usize, const CH: usize> {
    pub graph: StableGraph<Node<N, CH>, ()>,
    pub order: Vec<(NodeIndex, Vec<NodeIndex>)>,
    pub destination: NodeIndex,
}

impl<const N: usize, const CH: usize> Graph3<N, CH> {
    pub fn new(nodes: usize, edges: usize) -> Self {
        let mut graph = StableGraph::with_capacity(nodes, edges);
        let n = Node::new(Box::new(Mix::new()), 1);
        let mul = Node::new(Box::new(Mul::new(0.5)), 1);
        let sin = Node::new(Box::new(SinOsc::new(440.0, 44100)), 1);
        let index_dest = graph.add_node(n);
        let index_mul = graph.add_node(mul);
        let index_mul2 = graph.add_node(Node::new(Box::new(Mul::new(0.5)), 1));
        let index_mul3 = graph.add_node(Node::new(Box::new(Mul::new(0.5)), 1));
        let index_mul4 = graph.add_node(Node::new(Box::new(Mul::new(0.5)), 1));
        let index_sin = graph.add_node(sin);

        // graph.add_edge(index_sin, index_dest, ());
        graph.add_edge(index_sin, index_mul, ());
        // graph.add_edge(index_mul, index_mul2, ());
        // graph.add_edge(index_mul2, index_mul3, ());
        // graph.add_edge(index_mul3, index_mul4, ());
        graph.add_edge(index_mul, index_dest, ());

        Self {
            graph,
            order: Vec::new(),
            destination: index_dest,
        }
    }

    pub fn update_order(&mut self) {
        let reversed_graph = Reversed(&self.graph);
        let dfs_post = DfsPostOrder::new(&reversed_graph, self.destination);
        // let index_order = dfs_post.iter(&reversed_graph).collect();
        let mut order = Vec::new();
        for index in dfs_post.iter(&reversed_graph) {
            let mut inputs = Vec::new();
            let neighbors = self.graph.neighbors_directed(index, Incoming);
            for neighbor in neighbors {
                inputs.push(neighbor);
            }
            order.push((index, inputs));
        }
        self.order = order;
    }

    pub fn yield_next_buffer(&mut self) -> anyhow::Result<&[f32]> {
        let mut input = Vec::new();
        let graph_nodes_ptr = &mut self.graph as *mut StableGraph<Node<N, CH>, ()>;
        for n in self.order.iter() {
            input.clear();
            for in_n in n.1.iter() {
                // Skip edges that connect the node to itself to avoid aliasing `node`.
                // todo
                let node_weight = unsafe {
                    (*graph_nodes_ptr)
                        .node_weight_mut(*in_n)
                        .expect("No node weight")
                };
                input.push(&mut node_weight.buffer);
            }

            let current = unsafe {
                (*graph_nodes_ptr)
                    .node_weight_mut(n.0)
                    .context("No node weight")?
            };
            current.process(&input);
        }
        let d = self
            .graph
            .node_weight_mut(self.destination)
            .context("No node weight")?;
        Ok(&d.buffer.data[0])
    }
}
