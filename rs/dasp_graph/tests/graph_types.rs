//! Check that we properly support the major petgraph types.
//!
//! We only need to know they compile.

#![cfg(feature = "node-boxed")]
#![allow(unreachable_code, unused_variables)]

use dasp_graph::{BoxedNode, NodeData};
use petgraph::visit::GraphBase;

#[test]
#[should_panic]
fn test_graph() {
    type Graph = petgraph::Graph<NodeData<BoxedNode<128>, 128>, (), petgraph::Directed, u32>;
    type Processor = dasp_graph::Processor<Graph, 128>;
    let mut g: Graph = unimplemented!();
    let mut p: Processor = unimplemented!();
    let n: <Graph as GraphBase>::NodeId = unimplemented!();
    p.process(&mut g, n);
}

#[test]
#[should_panic]
fn test_stable_graph() {
    type Graph = petgraph::stable_graph::StableGraph<
        NodeData<BoxedNode<128>, 128>,
        (),
        petgraph::Directed,
        u32,
    >;
    type Processor = dasp_graph::Processor<Graph, 128>;
    let mut g: Graph = unimplemented!();
    let mut p: Processor = unimplemented!();
    let n: <Graph as GraphBase>::NodeId = unimplemented!();
    p.process(&mut g, n);
}
