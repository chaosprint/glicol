use dasp_graph::{Buffer, Input, Node};

pub struct Mul {
    pub mul: f32,
    has_mod: bool
}
impl Mul {
    pub fn new(mul: String) -> Self {
        let para = mul.parse::<f32>();
        if para.is_ok() {
            Self {mul: para.unwrap(), has_mod: false}
        } else {
            Self {mul: 0.0, has_mod: true}
        }
    }
}
impl Node for Mul {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        if !self.has_mod {
            if inputs.len() > 0 {
                // let buf = &mut inputs[0].buffers();
                // output[0] = buf[0].clone();
                output[0] = inputs[0].buffers()[0].clone();
                output[0].iter_mut().for_each(|s| *s = *s * self.mul as f32);
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
    pub increament: f64
}
impl Add {
    pub fn new(increament: f64) -> Self {
        Self { increament }
    }
}
impl Node for Add {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        if inputs.len() > 0 {
            output[0] = inputs[0].buffers()[0].clone();
            output[0].iter_mut().for_each(|s| *s = *s + self.increament as f32);
        }
    }
}