use super::super::*;

pub struct Add {
    pub inc: f32
}

impl Add {
    pub fn new(inc: f32) -> GlicolNodeData {
        mono_node!( Self { inc } )
    }
}

#[macro_export]
macro_rules! add {
    () => {
        Add::new(0.0)
    };

    ($data: expr) => {
        Add::new($data)
    };
}

impl Node<128> for Add {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        if inputs.len() > 1 {
            let buf = &mut inputs[0].buffers();
            let mod_buf = &mut inputs[1].buffers();
            for i in 0..128 {
                output[0][i] = mod_buf[0][i] + buf[0][i];
            }
        } else {
            output[0] = inputs[0].buffers()[0].clone();
            output[0].iter_mut().for_each(|s| *s = *s + self.inc as f32);
        }
    }
}