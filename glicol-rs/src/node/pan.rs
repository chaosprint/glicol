use dasp_graph::{Buffer, Input, Node};
use super::super::{Rule, NodeData, BoxedNodeSend,
    NodeResult, EngineError, midi_or_float};
use pest::iterators::Pairs;

pub struct Pan {
    pan: f32,
    has_mod: bool
}

impl Pan {
    pub fn new(paras: &mut Pairs<Rule>) -> NodeResult {

        let pan = paras.as_str().to_string();
        // .to_string().chars().filter(|c| !c.is_whitespace()).collect()
        // let b = Buffer::default();

        if pan.parse::<f32>().is_ok() {
            let f = midi_or_float(pan);
            // println!("{}", f);
            return Ok((NodeData::new2(BoxedNodeSend::new(Self {
                pan: f,
                has_mod: false
            })), vec![]))
        } else {
            return Ok((NodeData::new2(BoxedNodeSend::new(Self { 
                pan: 0.0,
                has_mod: true
            })), vec![pan]))
        }
    }
}

impl Node<128> for Pan {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        
        if self.has_mod {
            assert!(inputs.len() > 0);
            let mod_buf = &mut inputs[0].buffers();

            match inputs[0].buffers().len() {
                1 => {
                    output[0] = inputs[1].buffers()[0].clone();
                    output[1] = inputs[1].buffers()[0].clone();
                },
                2 => {
                    output[0] = inputs[1].buffers()[0].clone();
                    output[1] = inputs[1].buffers()[1].clone();
                },
                _ => {unimplemented!()}
            };
            
            for i in 0..128 {
                let p = mod_buf[0][i];
                output[0][i] *= 1.0 - (p+1.)/2.;
                output[1][i] *= (p+1.)/2.;
            }
            
        } else {
            match inputs[0].buffers().len() {
                1 => {
                    let mut l = inputs[0].buffers()[0].clone();
                    let mut r = l.clone();
                    l.iter_mut().for_each(|s| *s = *s * (1.0 -(self.pan+1./2.)) );
                    r.iter_mut().for_each(|s| *s = *s * (self.pan+1./2.));
                    output[0] = l;
                    output[1] = r;
                },
                2 => {
                    output[0] = inputs[0].buffers()[0].clone();
                    output[1] = inputs[0].buffers()[1].clone();
                    output[0].iter_mut().for_each(|s| *s = *s * (1.0 -(self.pan+1./2.)));
                    output[1].iter_mut().for_each(|s| *s = *s * (self.pan+1./2.));
                },
                _ => {panic!()}
            }
        }
    }
}

pub struct Mix2 {}
impl Mix2 {
    pub fn new(paras: &mut Pairs<Rule>) -> NodeResult {
        let para_a: String = paras.next().unwrap().as_str().to_string();
        let para_b: String = paras.next().unwrap().as_str().to_string();        
        return Ok((NodeData::new2(BoxedNodeSend::new(Self {})), vec![para_a, para_b]))
    }
}

impl Node<128> for Mix2 {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        // let _clock = inputs[2].clone();
        let left = inputs[1].buffers()[0].clone();
        let right = inputs[0].buffers()[0].clone();
        output[0] = left;
        output[1] = right;
    }
}