use super::super::*;

pub struct Add {
    pub inc: f32
}

impl Add {
    pub fn new(inc: f32) -> GlicolNodeData {
        mono_node!( Self { inc } )
    }
}


impl Node<128> for Add {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        let l = inputs.len();
        if l < 1 { return ()};
        let has_clock = inputs[l-1].buffers()[0][0] % 128. == 0. && inputs[l-1].buffers()[0][1] == 0.;

        if l - has_clock as usize > 1 { // has mod
            let buf = &mut inputs[0].buffers();
            let mod_buf = &mut inputs[1].buffers();
            for i in 0..128 {
                output[0][i] = mod_buf[0][i] + buf[0][i];
            }
        } else { // no mod          
            output[0] = inputs[0].buffers()[0].clone();
            output[0].iter_mut().for_each(|s| *s = *s + self.inc as f32);
        }
    }
}