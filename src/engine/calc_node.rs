
use dasp_graph::{Buffer, Input, Node};

pub struct Mul {
    pub mul: String
}
impl Mul {
    pub fn new(mul: String) -> Mul {
        Mul { mul }
    }
}
impl Node for Mul {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        let num = self.mul.parse::<f64>();
        if num.is_ok() {
            if inputs.len() > 0 {
                let buf = &mut inputs[0].buffers();
                output[0] = buf[0].clone(); // write
                output[0].iter_mut().for_each(|s| *s = *s * num.clone().unwrap() as f32);
            }
        } else {
            if inputs.len() > 1 {
                let buf = &mut inputs[0].buffers();
                let mod_buf = &mut inputs[1].buffers();
                // output[0] = buf[0].clone();
                output[0].clone_from_slice(&buf[0]);
                // for i in 0..output[0].len() {
                for i in 0..64 {
                    output[0][i] *= mod_buf[0][i];
                    // output[0].iter_mut().for_each(|s| *s = *s * 0.9 as f32);
                }
                
            }
        }
    }
}


pub struct Add {
    pub add: f64
}
impl Add {
    pub fn new(add: f64) -> Add {
        Add { add }
    }
}
impl Node for Add {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        if inputs.len() > 0 {
            let buf = &mut inputs[0].buffers();
            output[0] = buf[0].clone();
            output[0].iter_mut().for_each(|s| *s = *s + self.add as f32);
        }
    }
}