use glicol_macro::*;
use glicol_synth::{SimpleGraph, mono_node, GlicolNodeData};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};

pub struct Plate {
    graph: SimpleGraph
}

impl Plate {
    pub fn new(mix: f32) -> GlicolNodeData {
        let mixdiff = 1. - mix;
        let graph = make_graph!{
            ~dry: ~input;
            ~wet: ~dry >> onepole 0.7
            >> delay 0.05 >> apfgain 0.004771 0.75 >> apfgain 0.003595 0.75
            >> apfgain 0.01272 0.625 >> apfgain 0.009307 0.625
            >> add ~back
            >> apfgain ~modu 0.7;
            ~modu: sin 0.1 >> mul 0.0055 >> add 0.0295;
            ~aa: ~wet >> delayn 394.0;
            ~ab: ~aa >> delayn 2800.0;
            ~ac: ~ab >> delayn 1204.0;
            ~ba: ~ac >> delayn 2000.0 >> onepole 0.1
            >> apfgain 0.007596 0.5;
            ~bb: ~ba >> apfgain 0.03578 0.5;
            ~bc: ~bb >> apfgain ~modu 0.5;
            ~ca: ~bc >> delayn 179.0;
            ~cb: ~ca >> delayn 2679.0;
            ~cc: ~cb >> delayn 3500.0 >> mul 0.3;
            ~da: ~cc >> apfgain 0.03 0.7 >> delayn 522.0;
            ~db: ~da >> delayn 2400.0;
            ~dc: ~db >> delayn 2400.0;
            ~ea: ~dc >> onepole 0.1 >> apfgain 0.0062 0.7;
            ~eb: ~ea >> apfgain 0.03492 0.7;
            ~fa: ~eb >> apfgain 0.0204 0.7 >> delayn 1578.0;
            ~fb: ~fa >> delayn 2378.0;
            ~back: ~fb >> delayn 2500.0 >> mul 0.3;
            
            ~subtract_left: ~bb >> add ~db >> add ~ea >> add ~fa >> mul -1.0;
            
            ~left: ~aa >> add ~ab >> add ~cb >> add ~subtract_left
            >> mul #mix >> add ~drym;
            
            ~sub_right: ~eb >> add ~ab >> add ~ba >> add ~ca >> mul -1.0;
            
            ~right: ~da >> add ~db >> add ~fb >> add ~sub_right
            >> mul #mix >> add ~drym;
            
            ~drym: ~dry >> mul #mixdiff;
            
            out: balance ~left ~right 0.5;
        };
        mono_node!( Self { graph } )
    }
}
//  ~am: sin #freq >> mul 0.3 >> add 0.5;

impl Node<128> for Plate {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {       
        let mut input = [0.0; 128];
        for i in 0..128 {
            input[i] = inputs[0].buffers()[0][i];
        }
        let out = self.graph.next_block(&mut input);
        for i in 0..128 {
            output[0][i] = out[i];
            // output[1][i] = out[i+128];
        }
    }
}

#[macro_export]
macro_rules! plate {
    ($data: expr) => {
        Plate::new($data)
    };
}
