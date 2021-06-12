use glicol_macro::*;
use glicol_synth::{SimpleGraph, mono_node, GlicolNodeData};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};

pub struct AmpLFO {
    graph: SimpleGraph
}

impl AmpLFO {
    pub fn new(freq: f32) -> GlicolNodeData {
        // let mul = (high - low) / 2.0;
        // assert!(mul > 0.0);
        // assert!(low >= 0.0);
        // let ad = mul + low;
        let graph = make_graph!{
            out: ~input >> mul ~am;
            ~am: sin #freq >> mul 0.3 >> add 0.5;
        };
        mono_node!( Self { graph } )
    }
}
//  ~am: sin #freq >> mul 0.3 >> add 0.5;

impl Node<128> for AmpLFO {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {       
        let mut input = [0.0; 128];
        for i in 0..128 {
            input[i] = inputs[0].buffers()[0][i];
        }
        // println!("inputs {:?}", input);
        let out = self.graph.next_block(&mut input);
        for i in 0..128 {
            output[0][i] = out[i];
            // output[1][i] = out[i+128];
        }
        // println!("out {:?}", out);
    }
}

#[macro_export]
macro_rules! amplfo {
    ($data: expr) => {
        AmpLFO::new($data)
    };
}
