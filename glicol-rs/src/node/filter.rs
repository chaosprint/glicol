use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError, handle_params};
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

type Fixed = ring_buffer::Fixed<Vec<f32>>;

pub struct Allpass {
    delay: f32,
    decay: f32,
    // gain: f32,
    bufx: Fixed,
    bufy: Fixed,
    sidechain_ids: Vec::<u8>
}

impl Allpass {

    handle_params!(
        {
            delay: 0.5,
            decay: 2.0
        }, [
            (
                delay, bufx, |d: f32| -> Fixed {
                    let size = (d / 1000.0 * 44100.0) as usize;
                    ring_buffer::Fixed::from(vec![0.0; size])
                }
            ), (
                delay, bufy, |d: f32| -> Fixed {
                    let size = (d / 1000.0 * 44100.0) as usize;
                    ring_buffer::Fixed::from(vec![0.0; size])
                }
            )
        ]
    );
    // pub fn new(paras: &mut Pairs<Rule>) -> 
    // Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

    //     let para_a: String = paras.next().unwrap().as_str().to_string();
    //     let para_b: String = paras.next().unwrap().as_str().to_string();
    //     // let para_c: String = paras.next().unwrap().as_str().to_string();

    //     let delay = para_a.parse::<f32>()?;
    //     let decay = para_b.parse::<f32>()?;
    //     // let gain = para_c.parse::<f32>()?;

    //     let mut sidechains = Vec::<String>::new();
    //     let mut which_param = Vec::<u8>::new();
    //     if !para_a.parse::<f32>().is_ok() {sidechains.push(para_a); which_param.push(0)};
    //     if !para_b.parse::<f32>().is_ok() {sidechains.push(para_b); which_param.push(1)};
    //     // if !para_c.parse::<f64>().is_ok() {sidechains.push(para_c); which_param.push(2)};
    //     let size = (delay / 1000.0 * 44100.0) as usize;

    //     Ok((NodeData::new1( BoxedNodeSend::new( Self {
    //         delay: delay,
    //         decay: decay,
    //         bufx: ring_buffer::Fixed::from(vec![0.0; size]),
    //         bufy: ring_buffer::Fixed::from(vec![0.0; size]),
    //         // gain: gain,
    //         // control: which_param
    //         sidechain_ids: vec![]
    //     })), sidechains))
    // }
}

impl Node for Allpass {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        // output[0] = inputs[0].buffers()[0].clone();

        // y(n) = -a * x(n) + x(n-D) + a * y(n-D)
        // a = exp(log(0.001) * D/t60).
        // let decay = (self.decay * 44100.0) as usize;
        let a = (0.001_f32.log10() * (self.delay / self.decay)).exp();

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
    delay_time: f32,
    gain: f32,
    forward: f32,
    back: f32,
    bufx: Fixed,
    bufy: Fixed,
    sidechain_ids: Vec::<u8>
}

impl Comb {
    handle_params!(
        {
            delay_time: 600.0,
            gain: 0.5,
            forward: 0.5,
            back: 0.5
        }, [
            (
                delay_time, bufx, |d: f32| -> Fixed {
                    let size = (d / 1000.0 * 44100.0) as usize;
                    ring_buffer::Fixed::from(vec![0.0; size])
                }
            ), (
                delay_time, bufy, |d: f32| -> Fixed {
                    let size = (d / 1000.0 * 44100.0) as usize;
                    ring_buffer::Fixed::from(vec![0.0; size])
                }
            )
        ]
    );
    // pub fn new(paras: &mut Pairs<Rule>) -> 
    // Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

    //     let para_a: String = paras.next().unwrap().as_str().to_string();
    //     let para_b: String = paras.next().unwrap().as_str().to_string();
    //     let para_c: String = paras.next().unwrap().as_str().to_string();
    //     let para_d: String = paras.next().unwrap().as_str().to_string();
        
    //     let delay_time = para_a.parse::<f64>()?;
    //     let gain = para_b.parse::<f64>()?;
    //     let forward = para_c.parse::<f64>()?;
    //     let back = para_d.parse::<f64>()?;

    //     let mut sidechains = Vec::<String>::new();
    //     if !para_a.parse::<f64>().is_ok() {sidechains.push(para_a);};
    //     if !para_b.parse::<f64>().is_ok() {sidechains.push(para_b);};
    //     if !para_c.parse::<f64>().is_ok() {sidechains.push(para_c);};
    //     if !para_d.parse::<f64>().is_ok() {sidechains.push(para_d);};
        
    //     // if !para_c.parse::<f64>().is_ok() {sidechains.push(para_c); which_param.push(2)};
    //     let size = (delay_time / 1000.0 * 44100.0) as usize;

    //     Ok((NodeData::new1( BoxedNodeSend::new( Self {
    //         delay_time,
    //         gain,
    //         forward,
    //         back,
    //         bufx: ring_buffer::Fixed::from(vec![0.0; size]),
    //         bufy: ring_buffer::Fixed::from(vec![0.0; size]),
    //     })), sidechains))
    // }
}

impl Node for Comb {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        // output[0] = inputs[0].buffers()[0].clone();

        let a = self.gain;
        let b = self.forward;
        let c = self.back;
        // println!("{:?}",self.bufx);

        for i in 0..64 {
            let xn = inputs[0].buffers()[0][0];
            let xn_d = self.bufx[0];
            let yn_d = self.bufy[0];
            let yn = a * xn + b * xn_d + c * yn_d;
            self.bufx.push(xn);
            self.bufy.push(yn);
            output[0][i] = yn;
        }
       

    }
}

pub struct OnePole {
    sidechain_ids: Vec<u8>,
    a: f32,
    y1: f32
}

impl OnePole {
    handle_params!({
        a: 0.9
    }, {
        y1: 0.0
    });
}

impl Node for OnePole {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        match self.sidechain_ids.len() {
            0 => {
                let input_sig = inputs[0].buffers()[0].clone();
                for i in 0..64 {
                    let y = input_sig[i] + self.a * self.y1;
                    output[0][i] = y;
                    self.y1 = y;
                }
            },
            1 => {
                let modulator = inputs[0].buffers()[0].clone();
                let input_sig = inputs[1].buffers()[0].clone();
                for i in 0..64 {
                    let y = input_sig[i] + modulator[i] * self.y1;
                    output[0][i] = y;
                    self.y1 = y;
                }
            },
            _ => unimplemented!()
        };
    }
}

pub struct AllpassGain {
    delay: f32,
    a: f32,
    bufx: Fixed,
    bufy: Fixed,
    sidechain_ids: Vec::<u8>
}

impl AllpassGain {
    handle_params!({
        delay: 5000.0,
        a: 0.5
    }, [
        (
            delay, bufx, |d: f32| -> Fixed {
                let size = (d / 1000.0 * 44100.0) as usize;
                ring_buffer::Fixed::from(vec![0.0; size])
            }
        ), (
            delay, bufy, |d: f32| -> Fixed {
                let size = (d / 1000.0 * 44100.0) as usize;
                ring_buffer::Fixed::from(vec![0.0; size])
            }
        )
    ]);
}

impl Node for AllpassGain {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        match self.sidechain_ids.len() {
            0 => {
                for i in 0..64 {
                    // println!("{:?}", self.buf);
                    let xn = inputs[0].buffers()[0][i];
                    let yn = -self.a * xn
                    + self.bufx[0] + self.a * self.bufy[0];
                    
                    // save new input to ring buffer
                    self.bufx.push(xn);
                    self.bufy.push(yn);
                    output[0][i] = yn;
                }
            },
            1 => {
                let insig = inputs[1].buffers()[0].clone();
                let modulator = inputs[0].buffers()[0].clone();
                let new_delay_samples = (modulator[0] / 44100.0) as usize;
                let length = self.bufx.len();
                
                for i in 0..64 {
                    // println!("{:?}", self.buf);
                    let xn = insig[i];
                    let yn = -self.a * xn
                    + self.bufx[0] + self.a * self.bufy[0];
                    
                    // save new input to ring buffer
                    self.bufx.push(xn);
                    self.bufy.push(yn);
                    output[0][i] = yn;
                    self.bufx.set_first(length - new_delay_samples);
                    self.bufy.set_first(length - new_delay_samples);
                }
            },
            _ => {}
        }
    }
}