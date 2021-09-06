/// deprecated

use glicol_macro::*;
use glicol_synth::{SimpleGraph, mono_node, GlicolNodeData};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};

pub struct AmpLFO<const N: usize> {
    graph: SimpleGraph<N>
}

impl<const N: usize> AmpLFO<N> {
    pub fn new(freq: f32) -> GlicolNodeData<N> {
        // let mul = (high - low) / 2.0;
        // assert!(mul > 0.0);
        // assert!(low >= 0.0);
        // let ad = mul + low;
        let graph = make_graph!{
            out: ~input >> mul ~am;
            ~am: sin #freq >> mul 0.3 >> add 0.5;
        };
        mono_node!( N, Self { graph } )
    }
}
//  ~am: sin #freq >> mul 0.3 >> add 0.5;

impl<const N: usize> Node<N> for AmpLFO<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {       
        let mut input = [0.0; N];
        for i in 0..N {
            input[i] = inputs[0].buffers()[0][i];
        }
        // println!("inputs {:?}", input);
        let out = self.graph.next_block(&mut input);
        for i in 0..N {
            output[0][i] = out[i];
            // output[1][i] = out[i+N];
        }
        // println!("out {:?}", out);
    }
}

#[macro_export]
macro_rules! amplfo{
    ($size: expr =>  $data: expr) => {
        AmpLFO::<$size>::new($data)
    };
}
