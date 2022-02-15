use dasp_graph::{Buffer, Input, Node};
use super::super::{ NodeData, GlicolNodeData, BoxedNodeSend, mono_node};

pub struct ConstSig<const N:usize> {
    val: f32
}

impl<const N:usize> ConstSig<N> {
    pub fn new(val: f32) -> GlicolNodeData<N> {
        mono_node! ( N, Self {val} )
    }
}

impl<const N:usize> Node<N> for ConstSig<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        let min_user_input = 0;
        let l = inputs.len();
        // println!("sin l is {}", l);
        let max_user_input = 1;
        if l < min_user_input {return ()};
        let has_clock = match l {
            0 => false,
            _ => inputs[l-1].buffers()[0][0] % 128. == 0. 
            && inputs[l-1].buffers()[0][1] == 0.
        };
        
        match l {
            0 => {
                for i in 0..N {
                    output[0][i] = self.val;
                }
            },
            1 => {
                // in standalone mode, no mechanism to prevent double processing
                // basic fm
                if has_clock {
                    for i in 0..N {
                        output[0][i] = self.val;
                    }
                } else {
                    let mod_buf = &mut inputs[0].buffers();
                    for i in 0..N {
                        output[0][i] = mod_buf[0][i]
                    }
                }
            },
            2 => {
                // has clock input or somehow mistakenly connected by users
                let mod_buf = &mut inputs[0].buffers();
                for i in 0..N {
                    output[0][i] = mod_buf[0][i]
                }
            },
            _ => return ()
        }

    }
}