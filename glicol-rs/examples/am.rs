// / This exmaple shows how you can use Glicol as a basic audio library
// / The backend is mainly dasp_graph which is further built on top of petgraph
// / There are many DSP code in Glicol that can be reused
// / But before you get started, you should know some basic APIs in the example

use glicol::Engine;
use glicol::node::oscillator::{SinOsc};
use glicol::node::source::ConstSig;
use glicol::node::operator::{Mul, Add};
use glicol::node::Para::Number;
use glicol::node::Para::Ref;

fn main () {
    let mut engine = Engine::new();

    // let i_source = engine.graph.add_node(SinOsc::new(Number(440.)));
    // let i_sourcemul = engine.graph.add_node(Mul::new(Number(0.5)));
    // let i_mod = engine.graph.add_node(SinOsc::new(Number(0.2)));
    // let i_modmul = engine.graph.add_node(Mul::new(Number(0.1)));
    // let i_modadd = engine.graph.add_node(Add::new(Number(0.5)));

    // engine.graph.add_edge(i_mod, i_modmul, ());
    // engine.graph.add_edge(i_modmul, i_modadd, ());
    // engine.graph.add_edge(i_source, i_sourcemul, ());

     // lazy evaluation
    // make_graph! {
    //     source = chain![SinOsc(440.), Mul(modulator)];
    //     modulator = chain![SinOsc(0.2), Mul(0.1), Add(0.5)];
    // };

    for (refname, nodelist) in &engine.node_by_chain {
        if refname.contains("~") {
            continue;
        }
        let last = nodelist.len() - 1;
        engine.processor.process(&mut engine.graph, nodelist[last].0);
    }

    // for _ in engine. {
    //     engine.processor.process(&mut engine.graph, source);
    //     println!("{:?}", engine.graph[source].buffers[0]);
    // }
}
    // this is really not ergomonic at all


//     // A more ergonomic way is to use the macro provided by Glicol
//     let index_mod = chain![SinOsc(1.2), Map([-1, 1, 200, 300])];
//     let index_out = chain![SinOsc(440.), Mul(index_mod)]

//     // audio callback
//     engine.graph.process();
//     println!("The first 128 samples: {:?}", engine.graph[index_out].buffers[0]);

//     Another way is using Glicol syntax
//     engine.set_code("out: sin 440 >> mul ~mod; ~mod: sin 1.2 >> map -1 1 200 300")
//     engine.make_graph()

//     callback
//     let buf = engine.next_stereo::<128>();
// }