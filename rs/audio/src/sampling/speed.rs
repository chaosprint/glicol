use dasp_graph::{Buffer, Input, Node};
use super::super::{ NodeData, BoxedNodeSend, GlicolNodeData, mono_node,};

pub struct Speed {
    speed: f32
}

impl Speed {
    pub fn new(speed: f32) -> GlicolNodeData {
        mono_node!(Self {speed})
    }
}

impl Node<128> for Speed {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {

        match inputs.len() {
            0 => output[0][0] = self.speed as f32,
            1 => {
                if inputs[0].buffers()[0][0] % 128. == 0. && inputs[0].buffers()[0][1] == 0. {
                    // is clock
                    output[0][0] = self.speed as f32;
                } else {
                    let mod_buf = &mut inputs[0].buffers();
                    output[0][0] = mod_buf[0][0];
                }
            },
            2 => {
                if inputs[1].buffers()[0][0] % 128. == 0. && inputs[1].buffers()[0][1] == 0. {
                    let mod_buf = &mut inputs[0].buffers();
                    output[0][0] = mod_buf[0][0];
                } else {
                    return ()
                }
            },
            _ => return ()
        }
    }
}