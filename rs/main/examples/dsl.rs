use dasp_graph::{NodeData, BoxedNodeSend};
use glicol::*;
use glicol::node::operation::{mul::*, add::*};
use glicol::node::oscillator::sin_osc::*;
use glicol::node::signal::const_sig::*;
use glicol::node::signal::dummy::Clock;

fn main() {

    // have to use =>, too bad
    // lazy_graph!(
    //     engine: {
    //         out: sin 440. 5. => mul 0.5
    //     }
    // );
}

// lazy_graph!(
//     graph: {
//         out: sin 440. >> mul ~side;
//         ~side: sin 10. >> mul 0.3 >> add 0.5;
//     }
// )
// buffers = graph.process();