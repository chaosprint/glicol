/// This exmaple shows how you can use Glicol as a basic audio library
/// The backend is mainly dasp_graph which is further built on top of petgraph
/// There are many DSP code in Glicol that can be reused
/// But before you get started, you should know some basic APIs in the example

use glicol::Engine;
use glicol::node::oscillator::{SinOsc};
use glicol::node::operator::{Mul};
use glicol::node::Para::Number as Num;

fn main () {
    let mut engine = Engine::new();
        
    /// You can use the APis provided by dasp_graph directly
    /// ```
    /// let i_sin_modulator = engine.graph.add_node(SinOsc(vec![Num(1.2)]));//modulator
    /// let i_sin_modulator = engine.graph.add_node(
    ///     Map( vec![Num(-1.0), Num(1.0), Num(100.0), Num(300.0)] )
    /// );
    /// engine.graph.add_edge(i_sin_modulator, i_sin_modulator);

    /// let i_source = engine.graph.add_node(SinOsc(vec![Num(440.)]));
    /// let i_mul = engine.graph.add_node(Mul(vec![]));
    /// engine.graph.add_edge(i_sin_a, i_mul);
    /// ```

    // A more ergonomic way is to use the macro provided by Glicol
    let index_mod = chain![SinOsc(1.2), Map([-1, 1, 200, 300])];
    let index_out = chain![SinOsc(440.), Mul(index_mod)]

    // audio callback
    engine.graph.process();
    println!("The first 128 samples: {:?}", engine.graph[index_out].buffers[0]);

    // Another way is using Glicol syntax
    // engine.set_code("out: sin 440 >> mul ~mod; ~mod: sin 1.2 >> map -1 1 200 300")
    // engine.make_graph()

    // callback
    // let buf = engine.next_stereo::<128>();
}

