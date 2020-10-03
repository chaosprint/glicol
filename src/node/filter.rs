use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend};

pub struct LPF {
    cutoff: f32,
    q: f32,
    has_mod: bool,
    x0: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl LPF {
    pub fn new(paras: &mut Pairs<Rule>) -> (NodeData<BoxedNodeSend>, Vec<String>) {

        // TODO: figure out paras handling
        // let mut paras = paras.next().unwrap().into_inner();
        // println!("paras {:?}", paras);

        // let seq = 

        let para_a: String = paras.next().unwrap().as_str().to_string();
        // .chars().filter(|c| !c.is_whitespace()).collect();

        // println!("'{}'", para_a);
        // ;

        // println!("{:?}", paras.as_str());
        let para_b: String = paras.next().unwrap().as_str().to_string();
        // .chars().filter(|c| !c.is_whitespace()).collect();

        // let cutoff = para_a.parse::<f32>();
        
        let q = para_b.parse::<f32>().unwrap();

        let mut destination = Vec::<String>::new();

        let cutoff = match para_a.parse::<f32>() {
            Ok(val) => val,
            Err(_why) => {destination.push(para_a.clone()); 1000.0}
        };
        (NodeData::new1( BoxedNodeSend::new( Self {
            cutoff: cutoff,
            q: q,
            has_mod: !para_a.parse::<f32>().is_ok(),
            x0: 0.,
            x1: 0.,
            x2: 0.,
            y1: 0.,
            y2: 0.,
        })), destination)
    }
}

impl Node for LPF {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        if !self.has_mod {
            let theta_c = 2.0 * std::f32::consts::PI * self.cutoff / 44100.0;
            let d = 1.0 / self.q;
            let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
            let gama = (0.5 + beta) * theta_c.cos();
            let a0 = (0.5 + beta - gama) / 2.0;
            let a1 = 0.5 + beta - gama;
            let a2 = (0.5 + beta - gama) / 2.0;
            let b1 = -2.0 * gama;
            let b2 = 2.0 * beta;
            assert!(inputs.len()>0, "no input to the filter");
            for i in 0..64 {
                // output[0][i] = inputs[0].buffers()[0][i];
                let x0 = inputs[0].buffers()[0][i];
                let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 
                - b1 * self.y1 - b2 * self.y2;

                output[0][i] = y;
                self.x2 = self.x1;
                self.x1 = x0;
                self.y2 = self.y1;
                self.y1 = y;
            }
        } else {
            // if inputs.len() > 1 {
                // let input_buf = &mut inputs[0].buffers();
                // let mod_buf = &mut inputs[1].buffers();
            // println!("input len{}", inputs.len());
            assert!(inputs.len()>1, "no sidechain info");
            
            let theta_c = 2.0 * std::f32::consts::PI * inputs[0].buffers()[0][0] / 44100.0;
            let d = 1.0 / self.q;
            let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
            let gama = (0.5 + beta) * theta_c.cos();
            let a0 = (0.5 + beta - gama) / 2.0;
            let a1 = 0.5 + beta - gama;
            let a2 = (0.5 + beta - gama) / 2.0;
            let b1 = -2.0 * gama;
            let b2 = 2.0 * beta;

            for i in 0..64 {
                // output[0][i] = inputs[0].buffers()[0][i];  
                let x0 = inputs[1].buffers()[0][i]; // strange order hard code, this should be 0
                let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 - b1 * self.y1 - b2 * self.y2;
                output[0][i] = y;
                self.x2 = self.x1;
                self.x1 = x0;
                self.y2 = self.y1;
                self.y1 = y;
            }
            // }
        }
    }
}

pub struct HPF {
    cutoff: f32,
    q: f32,
    has_mod: bool,
    x0: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl HPF {
    pub fn new(paras: &mut Pairs<Rule>) -> (NodeData<BoxedNodeSend>, Vec<String>) {

        let para_a: String = paras.next().unwrap().as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        let para_b: String = paras.next().unwrap().as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        // let cutoff = para_a.parse::<f32>();
        let q = para_b.parse::<f32>().unwrap();

        let mut destination = Vec::<String>::new();

        let cutoff = match para_a.parse::<f32>() {
            Ok(val) => val,
            Err(_why) => {destination.push(para_a.clone()); 1000.0}
        };
        (NodeData::new1( BoxedNodeSend::new( Self {
            cutoff: cutoff,
            q: q,
            has_mod: !para_a.parse::<f32>().is_ok(),
            x0: 0.,
            x1: 0.,
            x2: 0.,
            y1: 0.,
            y2: 0.,
        })), destination)
    }
}

impl Node for HPF {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        if !self.has_mod {
            let theta_c = 2.0 * std::f32::consts::PI * self.cutoff / 44100.0;
            let d = 1.0 / self.q;
            let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
            let gama = (0.5 + beta) * theta_c.cos();
            let a0 = (0.5 + beta + gama) / 2.0;
            let a1 = -0.5 - beta - gama;
            let a2 = (0.5 + beta + gama) / 2.0;
            let b1 = -2.0 * gama;
            let b2 = 2.0 * beta;
            // let c0 = 1.0;
            // let d0 = 0.0;
            // let y = 
            if inputs.len() > 0 {
                for i in 0..64 {
                    // output[0][i] = inputs[0].buffers()[0][i];
                    let x0 = inputs[0].buffers()[0][i];
                    let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 - b1 * self.y1 - b2 * self.y2;
                    output[0][i] = y;
                    self.x2 = self.x1;
                    self.x1 = x0;
                    self.y2 = self.y1;
                    self.y1 = y;
                }
            }
        } else {
            if inputs.len() > 1 {
                // let input_buf = &mut inputs[0].buffers();
                // let mod_buf = &mut inputs[1].buffers();

                // // mod_buf[0][i]
                // inputs[0] is the modulator... strange
                let theta_c = 2.0 * std::f32::consts::PI * inputs[0].buffers()[0][0] / 44100.0;
                let d = 1.0 / self.q;
                let beta = 0.5 * (1.0-d*theta_c.sin()/2.0) / (1.0+d*theta_c.sin()/2.0);
                let gama = (0.5 + beta) * theta_c.cos();
                let a0 = (0.5 + beta + gama) / 2.0;
                let a1 = -0.5 - beta - gama;
                let a2 = (0.5 + beta + gama) / 2.0;
                let b1 = -2.0 * gama;
                let b2 = 2.0 * beta;

                for i in 0..64 {
                    // output[0][i] = inputs[0].buffers()[0][i];  
                    let x0 = inputs[1].buffers()[0][i]; // strange order hard code
                    let y = a0 * self.x0 + a1 * self.x1 + a2 * self.x2 - b1 * self.y1 - b2 * self.y2;
                    output[0][i] = y;
                    self.x2 = self.x1;
                    self.x1 = x0;
                    self.y2 = self.y1;
                    self.y1 = y;
                }
            }
        }
    }
}