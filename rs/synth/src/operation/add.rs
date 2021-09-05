use super::super::*;

pub struct Add<const N:usize> {
    pub inc: f32
}

impl<const N:usize> Add<N> {
    pub fn new(inc: f32) -> GlicolNodeData<N> {
        mono_node!( N, Self { inc } )
    }
}


impl<const N:usize> Node<N> for Add<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        let max_user_input = 2;
        let min_user_input = 1;
        let l = inputs.len();
        if l < min_user_input { return ()};
        let has_clock = match l {
            0 => false,
            _ => inputs[l-1].buffers()[0][0] as usize % N == 0
            && inputs[l-1].buffers()[0][1] == 0.
        };
        // println!("l - has_clock as usize is {:?}", l - has_clock as usize);
        // println!("has_clock is {:?}",  has_clock);
        // println!(" self.inc {:?}",  self.inc);
        // println!("inputs to add node {:?}", inputs);
        match l {
            1 => {
                output[0] = inputs[0].buffers()[0].clone();
                output[0].iter_mut().for_each(|s| *s = *s + self.inc as f32);
            },
            2 => {
                if has_clock {
                    output[0] = inputs[0].buffers()[0].clone();
                    output[0].iter_mut().for_each(|s| *s = *s + self.inc as f32);
                    // println!("output[0] should be {:?}", output[0]);
                } else {
                    let buf = &mut inputs[0].buffers();
                    let mod_buf = &mut inputs[1].buffers();
                    for i in 0..N {
                        output[0][i] = mod_buf[0][i] + buf[0][i];
                    }
                }
            },
            3 => {
                // panic!();
                let buf = &mut inputs[0].buffers();
                let mod_buf = &mut inputs[1].buffers();
                for i in 0..N {
                    output[0][i] = mod_buf[0][i] + buf[0][i];
                }
            },
            _ => return ()
        };
        // println!("output from add node {:?}", output);
    }
}
