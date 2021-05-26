use dasp_graph::{NodeData, BoxedNodeSend};
use glicol::*;
use glicol::node::speed::*;
use glicol::node::seq::*;
use glicol::node::sampler::*;
use glicol::node::imp::*;
use glicol::node::mul::*;

fn main() {
    let mut e = Engine::new(44100);
    let i = e.graph.add_node(speed!(4410.0));
    // let i = e.graph.add_node(seq!({pattern: "60 _60"}));
    e.process(i);
    println!("{:?}", e.graph[i].buffers);
}