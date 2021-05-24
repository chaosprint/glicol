// / This exmaple shows how you can use Glicol as a basic audio library
// / The backend is mainly dasp_graph which is further built on top of petgraph
// / There are many DSP code in Glicol that can be reused
// / But before you get started, you should know some basic APIs in the example

use glicol::*;
use glicol::node::sin_osc::*;
use glicol::node::add::*;
use glicol::node::mul::*;

fn main () {
    let mut engine = Engine::new(44100);
    // return a vec of nodeindex
    let i_mod = engine.make_chain(vec![sin_osc!({freq: 10.0}), mul!(0.5), add!(0.5)]);
    let out = engine.make_chain(vec![sin_osc!({freq: 440.0}), mul!()]);
    engine.make_edge(i_mod[2], out[1]);
    engine.process(out[1]); // this is a simplified method for calling processor on graph
    println!("{:?}", engine.graph[out[1]].buffers);
}