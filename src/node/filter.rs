use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError};
use dasp_ring_buffer as ring_buffer;

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
    pub fn new(paras: &mut Pairs<Rule>) -> 
    Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

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
        Ok((NodeData::new1( BoxedNodeSend::new( Self {
            cutoff: cutoff,
            q: q,
            has_mod: !para_a.parse::<f32>().is_ok(),
            x0: 0.,
            x1: 0.,
            x2: 0.,
            y1: 0.,
            y2: 0.,
        })), destination))
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
    pub fn new(paras: &mut Pairs<Rule>) -> 
    Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

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
        Ok((NodeData::new1( BoxedNodeSend::new( Self {
            cutoff: cutoff,
            q: q,
            has_mod: !para_a.parse::<f32>().is_ok(),
            x0: 0.,
            x1: 0.,
            x2: 0.,
            y1: 0.,
            y2: 0.,
        })), destination))
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

pub struct Allpass {
    delay: f64,
    decay: f64,
    // gain: f32,
    bufx: ring_buffer::Fixed<Vec<f32>>,
    bufy: ring_buffer::Fixed<Vec<f32>>,
    control: Vec::<u8>
}

impl Allpass {
    pub fn new(paras: &mut Pairs<Rule>) -> 
    Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

        let para_a: String = paras.next().unwrap().as_str().to_string();
        let para_b: String = paras.next().unwrap().as_str().to_string();
        // let para_c: String = paras.next().unwrap().as_str().to_string();

        let delay = para_a.parse::<f64>()?;
        let decay = para_b.parse::<f64>()?;
        // let gain = para_c.parse::<f32>()?;

        let mut sidechains = Vec::<String>::new();
        let mut which_param = Vec::<u8>::new();
        if !para_a.parse::<f64>().is_ok() {sidechains.push(para_a); which_param.push(0)};
        if !para_b.parse::<f64>().is_ok() {sidechains.push(para_b); which_param.push(1)};
        // if !para_c.parse::<f64>().is_ok() {sidechains.push(para_c); which_param.push(2)};
        let size = (delay / 1000.0 * 44100.0) as usize;

        Ok((NodeData::new1( BoxedNodeSend::new( Self {
            delay: delay,
            decay: decay,
            bufx: ring_buffer::Fixed::from(vec![0.0; size]),
            bufy: ring_buffer::Fixed::from(vec![0.0; size]),
            // gain: gain,
            control: which_param
        })), sidechains))
    }
}

impl Node for Allpass {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        // output[0] = inputs[0].buffers()[0].clone();

        // y(n) = -a * x(n) + x(n-D) + a * y(n-D)
        // a = exp(log(0.001) * D/t60).
        // let decay = (self.decay * 44100.0) as usize;
        let a = (0.001_f64.log10() * (self.delay / self.decay)).exp();

        for i in 0..64 {
            // println!("{:?}", self.buf);
            let xn = inputs[0].buffers()[0][i];
            let yn = -a as f32 * xn + self.bufx[0] + a as f32 * self.bufy[0];
            // save new input to ring buffer
            self.bufx.push(xn);
            self.bufy.push(yn);
            output[0][i] = yn;
        }


    }
}

pub struct Comb {
    delay_time: f64,
    gain: f64,
    forward: f64,
    back: f64,
    bufx: ring_buffer::Fixed<Vec<f64>>,
    bufy: ring_buffer::Fixed<Vec<f64>>,
}

impl Comb {
    pub fn new(paras: &mut Pairs<Rule>) -> 
    Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

        let para_a: String = paras.next().unwrap().as_str().to_string();
        let para_b: String = paras.next().unwrap().as_str().to_string();
        let para_c: String = paras.next().unwrap().as_str().to_string();
        let para_d: String = paras.next().unwrap().as_str().to_string();

        let delay_time = para_a.parse::<f64>()?;
        let gain = para_b.parse::<f64>()?;
        let forward = para_c.parse::<f64>()?;
        let back = para_d.parse::<f64>()?;

        let mut sidechains = Vec::<String>::new();
        if !para_a.parse::<f64>().is_ok() {sidechains.push(para_a);};
        if !para_b.parse::<f64>().is_ok() {sidechains.push(para_b);};
        if !para_c.parse::<f64>().is_ok() {sidechains.push(para_c);};
        if !para_d.parse::<f64>().is_ok() {sidechains.push(para_d);};
        
        // if !para_c.parse::<f64>().is_ok() {sidechains.push(para_c); which_param.push(2)};
        let size = (delay_time / 1000.0 * 44100.0) as usize;

        Ok((NodeData::new1( BoxedNodeSend::new( Self {
            delay_time,
            gain,
            forward,
            back,
            bufx: ring_buffer::Fixed::from(vec![0.0; size]),
            bufy: ring_buffer::Fixed::from(vec![0.0; size]),
        })), sidechains))
    }
}

impl Node for Comb {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        // output[0] = inputs[0].buffers()[0].clone();

        let a = self.gain;
        let b = self.forward;
        let c = self.back;
        // println!("{:?}",self.bufx);

        for i in 0..64 {
            let xn = inputs[0].buffers()[0][0] as f64;
            let xn_d = self.bufx[0];
            let yn_d = self.bufy[0];
            let yn = a * xn + b * xn_d + c * yn_d;
            self.bufx.push(xn);
            self.bufy.push(yn);
            output[0][i] = yn as f32;
        }
       

    }
}