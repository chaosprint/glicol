use dasp_graph::{Buffer, Input, Node};
use super::super::{HashMap, NodeData, BoxedNodeSend, GlicolNodeData, mono_node,};

// pub struct

#[derive(Clone, Debug, Default)]
pub struct Sequencer<const N:usize> {
    events: Vec<(f64, String)>,
    speed: f32,
    bpm: f32,
    sr: usize,
    pub step: usize,
    sidechain_lib: HashMap<String, usize>,
}

impl<const N:usize> Sequencer<N> {
    pub fn new() -> Self {
        Self { speed: 1., bpm: 120., sr: 44100, ..Self::default()}
    }
    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self}
    }
    pub fn bpm(self, bpm: f32) -> Self {
        Self {bpm, ..self}
    }

    pub fn sidechain_lib(self, sidechain_lib: HashMap<String, usize>) -> Self {
        Self {sidechain_lib, ..self}
    }

    pub fn events(self, events: Vec::<(f64, String)>) -> Self {
        Self {events, ..self}
    }

    pub fn build(self) -> GlicolNodeData<N> {
        mono_node!( N, self)
    }

}

/// The Sequencer is probably the most complicated node in Glicol
/// The inputs can be clock, speed or many sidechains
impl<const N:usize> Node<N> for Sequencer<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        let mut has_speed_input = false;
        let mut has_clock = false;        

        if inputs.len() == 1 {
            if inputs[0].buffers()[0][0] as usize % N == 0 && inputs[0].buffers()[0][1] == 0. {
                self.step = inputs[0].buffers()[0][0] as usize;
                has_clock = true;
            } else if inputs[0].buffers()[0][0] as usize % N > 0 && inputs[0].buffers()[0][1] == 0. {
                self.speed = inputs[0].buffers()[0][0];
                has_speed_input = true;
            }
        } else if inputs.len() > 1 {
            let m = inputs.len() - 1;
            if inputs[m].buffers()[0][0] as usize % N == 0 && inputs[m].buffers()[0][1] == 0. {
                self.step = inputs[m].buffers()[0][0] as usize;
                has_clock = true;
                if inputs[m-1].buffers()[0][0] as usize % N  > 0 && inputs[m-1].buffers()[0][1] == 0. {
                    self.speed = inputs[m-1].buffers()[0][0];
                    has_speed_input = true;
                }
            } else if inputs[m].buffers()[0][0] as usize % N > 0 && inputs[m].buffers()[0][1] == 0. {
                self.speed = inputs[m].buffers()[0][0];
                has_speed_input = true;
            }
        }

        // println!("{}{}", has_clock, has_speed_input);
        // println!("has clock? {} has speed input? {}", has_clock, has_speed_input);
        // let relative_time = event.0;
        // let relative_pitch = event.1; a ratio for midi 60 freq
        let bar_length = 240.0 / self.bpm as f64 * self.sr as f64 / self.speed as f64;
        for i in 0..N {
            output[0][i] = 0.0;

            for event in &self.events {
                if (self.step % (bar_length as usize)) == ((event.0 * bar_length) as usize) {
                    // println!("{}", (event.0 * bar_length) );
                    let midi = match event.1.parse::<f32>() {
                        Ok(val) => val,
                        Err(_) => {
                            // let len = inputs.len();
                            // there are cases:
                            // - no speed input, but has several sidechains
                            // - one speed input, no sidechain,
                            // - one speed input. several sidechains
                            // if inputs.len() as i8 - 1 > self.sidechain_lib[&event.1] as i8 {
                            let index = inputs.len() - 1 - has_clock as usize - has_speed_input as usize
                            - self.sidechain_lib[&event.1];
                            inputs[index].buffers()[0][i]
                        }
                    };

                    if midi == 0.0 {
                        output[0][i] = 0.0
                    } else {
                        output[0][i] = 2.0f32.powf((midi - 60.0)/12.0)
                    }
                }
            }
            self.step += 1;
        }
    }
}
