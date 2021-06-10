use super::super::{GlicolNodeData, mono_node};
use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};

pub struct Mul {
    mul: f32,
    transit_begin: f32,
    transit_end: f32,
    transit_index: usize,
    transit: bool,
    window: Vec<f64>,
}

impl Mul {
    pub fn new(mul: f32) -> GlicolNodeData {
        return mono_node!( Self {
            mul,
            transit_begin: 0.0,
            transit_end: 0.0,
            transit_index: 0,
            transit: false,
            window: apodize::hanning_iter(2048).collect::<Vec<f64>>(),
        })
    }
}

impl Node<128> for Mul {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        // println!("inputs from mul {:?}", inputs);
        
        let min_user_input = 1;
        let max_user_input = 2;
        let l = inputs.len();
        if l < min_user_input { return ()};
        let has_clock = match l {
            0 => false,
            _ => inputs[l-1].buffers()[0][0] % 128. == 0. 
            && inputs[l-1].buffers()[0][1] == 0.
        };

        match l {
            1 => {
                output[0] = inputs[0].buffers()[0].clone();
                output[0].iter_mut().for_each(|s| *s = *s * self.mul as f32);
            },
            2 => {
                if has_clock {
                    output[0] = inputs[0].buffers()[0].clone();
                    output[0].iter_mut().for_each(|s| *s = *s * self.mul as f32);
                } else {
                    let buf = &mut inputs[1].buffers();
                    let mod_buf = &mut inputs[0].buffers();
            
                    self.transit = self.transit_begin != mod_buf[0][0]
                    && mod_buf[0][0] == mod_buf[0][63];
            
                    if self.transit {
                        self.transit_end = mod_buf[0][0];
                    }
            
                    let distance = self.transit_begin - self.transit_end;
            
                    if self.transit_index == 1024 {
                        self.transit_index = 0;
                        self.transit_begin = self.transit_end.clone();
                        self.transit = false;
                    }
            
                    for i in 0..128 {
                        output[0][i] = match self.transit {
                            true => {
                                let phase = self.transit_begin - 
                                self.window[self.transit_index] as f32 * distance;
                                self.transit_index += 1;
                                phase * buf[0][i]
                            },
                            false => {
                                mod_buf[0][i] * buf[0][i]
                            }
                        };
                    }
                }
            },
            3 => {
                if has_clock {
                    let buf = &mut inputs[1].buffers();
                    let mod_buf = &mut inputs[0].buffers();
            
                    self.transit = self.transit_begin != mod_buf[0][0]
                    && mod_buf[0][0] == mod_buf[0][63];
            
                    if self.transit {
                        self.transit_end = mod_buf[0][0];
                    }
            
                    let distance = self.transit_begin - self.transit_end;
            
                    if self.transit_index == 1024 {
                        self.transit_index = 0;
                        self.transit_begin = self.transit_end.clone();
                        self.transit = false;
                    }
            
                    for i in 0..128 {
                        output[0][i] = match self.transit {
                            true => {
                                let phase = self.transit_begin - 
                                self.window[self.transit_index] as f32 * distance;
                                self.transit_index += 1;
                                phase * buf[0][i]
                            },
                            false => {
                                mod_buf[0][i] * buf[0][i]
                            }
                        };
                    }
                } else {
                    return ()
                }
            },
            _ => return ()
        }
        // println!("output from mul {:?}", output);
    }
}