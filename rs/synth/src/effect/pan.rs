use dasp_graph::{Buffer, Input, Node};
use super::super::{GlicolNodeData, NodeData, BoxedNodeSend, stereo_node};

pub struct Pan<const N:usize> {
    pan: f32
}

impl<const N:usize> Pan<N> {
    pub fn new(pan: f32) -> GlicolNodeData<N> {
        stereo_node!( N, Self { pan } )
    }
}

impl<const N:usize> Node<N> for Pan<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {

        let l = inputs.len();
        if l < 1 { return ()};
        
        // not strict
        let has_clock = inputs[l-1].buffers()[0][0] as usize % N == 0 && inputs[l-1].buffers()[0][1] == 0.;

        // println!("{}{}",l ,has_clock );
        if l - has_clock as usize > 1 { // has mod

            let mod_buf = &mut inputs[0].buffers();

            match inputs[0].buffers().len() {
                1 => {
                    output[0] = inputs[1].buffers()[0].clone();
                    output[1] = inputs[1].buffers()[0].clone();
                },
                2 => {
                    output[0] = inputs[1].buffers()[0].clone();
                    output[1] = inputs[1].buffers()[1].clone();
                },
                _ => {unimplemented!()}
            };
            
            for i in 0..N {
                let p = mod_buf[0][i];
                output[0][i] *= 1.0 - (p+1.)/2.;
                output[1][i] *= (p+1.)/2.;
            }

        } else {
            match inputs[0].buffers().len() {
                1 => {
                    let mut l = inputs[0].buffers()[0].clone();
                    let mut r = l.clone();
                    l.iter_mut().for_each(|s| *s = *s * (1.0 - ((self.pan + 1.0) /2.0 )));
                    r.iter_mut().for_each(|s| *s = *s * ((self.pan + 1.0) / 2.0));
                    output[0] = l;
                    output[1] = r;
                },
                2 => {
                    output[0] = inputs[0].buffers()[0].clone();
                    output[1] = inputs[0].buffers()[1].clone();
                    output[0].iter_mut().for_each(|s| *s = *s * (1.0 - ( (self.pan + 1.0) /2.0 )));
                    output[1].iter_mut().for_each(|s| *s = *s * ((self.pan + 1.0) /2.0));
                },
                _ => {unimplemented!()}
            }
        }
        
        // if false {
        //     assert!(inputs.len() > 0);
        //     let mod_buf = &mut inputs[0].buffers();

        //     match inputs[0].buffers().len() {
        //         1 => {
        //             output[0] = inputs[1].buffers()[0].clone();
        //             output[1] = inputs[1].buffers()[0].clone();
        //         },
        //         2 => {
        //             output[0] = inputs[1].buffers()[0].clone();
        //             output[1] = inputs[1].buffers()[1].clone();
        //         },
        //         _ => {unimplemented!()}
        //     };
            
        //     for i in 0..N {
        //         let p = mod_buf[0][i];
        //         output[0][i] *= 1.0 - (p+1.)/2.;
        //         output[1][i] *= (p+1.)/2.;
        //     }
            
        // } else {
        //     match inputs[0].buffers().len() {
        //         1 => {
        //             let mut l = inputs[0].buffers()[0].clone();
        //             let mut r = l.clone();
        //             l.iter_mut().for_each(|s| *s = *s * (1.0 -(self.pan+1./2.)) );
        //             r.iter_mut().for_each(|s| *s = *s * (self.pan+1./2.));
        //             output[0] = l;
        //             output[1] = r;
        //         },
        //         2 => {
        //             output[0] = inputs[0].buffers()[0].clone();
        //             output[1] = inputs[0].buffers()[1].clone();
        //             output[0].iter_mut().for_each(|s| *s = *s * (1.0 -(self.pan+1./2.)));
        //             output[1].iter_mut().for_each(|s| *s = *s * (self.pan+1./2.));
        //         },
        //         _ => {panic!()}
        //     }
        // }
    }
}