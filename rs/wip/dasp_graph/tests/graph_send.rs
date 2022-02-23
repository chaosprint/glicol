//! Small test to make sure `Processor` and graph types stay `Send`.
//!
//! We only need to know they compile.

#![cfg(feature = "node-boxed")]
#![allow(unreachable_code, unused_variables)]

use dasp_graph::{BoxedNodeSend, NodeData};
use petgraph::visit::GraphBase;

#[test]
#[should_panic]
fn test_graph_send() {
    type Graph = petgraph::Graph<NodeData<BoxedNodeSend<128>, 128>, (), petgraph::Directed, u32>;
    type Processor = dasp_graph::Processor<Graph, 128>;
    let mut g: Graph = unimplemented!();
    let mut p: Processor = unimplemented!();
    let n: <Graph as GraphBase>::NodeId = unimplemented!();

    std::thread::spawn(move || {
        p.process(&mut g, n);
    });
}
