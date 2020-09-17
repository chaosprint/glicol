
use dasp_graph::{Buffer, Input, Node};

pub struct Mul {
    pub mul: String
}
impl Mul {
    pub fn new(mul: String) -> Self {
        Self { mul }
    }
}
impl Node for Mul {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        let num = self.mul.parse::<f64>();
        if num.is_ok() {
            if inputs.len() > 0 {
                let buf = &mut inputs[0].buffers();
                output[0] = buf[0].clone(); // write

                // can we avoid this clone?
                output[0].iter_mut().for_each(|s| *s = *s * num.clone().unwrap() as f32);
            }
        } else {
            if inputs.len() > 1 {
                let buf = &mut inputs[0].buffers();
                let mod_buf = &mut inputs[1].buffers();
                for i in 0..64 {
                    output[0][i] = mod_buf[0][i] * buf[0][i];
                }
            }
        }
    }
}

pub struct Add {
    pub add: f64
}
impl Add {
    pub fn new(add: f64) -> Self {
        Self { add }
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