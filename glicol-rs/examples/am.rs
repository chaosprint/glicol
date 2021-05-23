// / This exmaple shows how you can use Glicol as a basic audio library
// / The backend is mainly dasp_graph which is further built on top of petgraph
// / There are many DSP code in Glicol that can be reused
// / But before you get started, you should know some basic APIs in the example

use glicol::Engine;
use glicol::node::oscillator::{SinOsc};
use glicol::node::operator::{Mul, Add};

fn main () {
    let mut engine = Engine::new(44100);

    let i_source = engine.graph.add_node(SinOsc::new("440"));
    let i_sourcemul = engine.graph.add_node(Mul::new(""));
    let i_mod = engine.graph.add_node(SinOsc::new("10"));
    let i_modmul = engine.graph.add_node(Mul::new("0.1"));
    let i_modadd = engine.graph.add_node(Add::new("0.6"));

    engine.graph.add_edge(i_source, i_sourcemul, ());
    engine.graph.add_edge(i_mod, i_modmul, ());
    engine.graph.add_edge(i_modmul, i_modadd, ());
    engine.graph.add_edge(i_modadd, i_sourcemul, ());

    // process
    engine.processor.process(&mut engine.graph, i_sourcemul);

    // fetch the output
    println!("{:?}", engine.graph[i_sourcemul].buffers[0]);

    let from = engine.new_node("sin", ["440"]);
    // let to = engine.new_node("mul", 0.5);
    // engine.chain([from, to, engine.output]);
    // let buffers = engine.process();
    // println!("{:?}", buffers[0])


    // lazy evaluation + dummy self.clock
    // make_graph! {
    //     source: [SinOsc(440.), Mul(_modulator)],
    //     _modulator: [SinOsc(0.2), Mul(0.1), Add(0.5)]
    // };

    // engine.process();
    // engine.buffers[0];

    // for _ in 0..2 { // 2 blocks
    //     let chains = engine.node_by_chain.iter().filter( |&(k, v)| {k.starts_with("_")});
    //     for (_, chain) in chains {
    //         let last = chain.len() - 1;
    //         engine.processor.process(&mut engine.graph, chain[last].0);
    //     }
        // engine.update_clock();
    // }

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