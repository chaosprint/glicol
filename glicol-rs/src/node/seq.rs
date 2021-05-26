use dasp_graph::{Buffer, Input, Node};
use super::super::{HashMap, NodeData, BoxedNodeSend, GlicolNodeData, mono_node,};

// pub struct

#[derive(Clone, Debug, Default)]
pub struct Sequencer {
    events: Vec<(f64, String)>,
    speed: f32,
    bpm: f32,
    sr: usize,
    pub step: usize,
    sidechain_lib: HashMap<String, usize>,
    sidechain_id: usize,
}

impl Sequencer {
    pub fn new() -> Self {
        Self { speed: 1., bpm: 120., sr: 44100, ..Self::default()}
    }
    pub fn sr(self, sr: usize) -> Self {
        Self { sr, ..self}
    }
    pub fn bpm(self, bpm: f32) -> Self {
        Self {bpm, ..self}
    }
    pub fn pattern(mut self, pattern: &str) -> Self {
        let mut events = Vec::<(f64, String)>::new();
        // let mut sidechains = Vec::<String>::new();
        // let mut sidechain_id = 0;
        // let mut sidechain_lib = HashMap::<String, usize>::new();
        let split: Vec<&str> = pattern.split(" ").collect();
        let len_by_space = split.len();
        let compound_unit = 1.0 / len_by_space as f64;

        for (i, compound) in split.iter().enumerate() {
            let c = compound.replace("_", "$_$");
            let notes = c.split("$").filter(|x|x!=&"").collect::<Vec<_>>();

            let notes_len = notes.len();

            // println!("len = {}", notes_len);

            for (j, x) in notes.iter().enumerate() {
                let relative_time = i as f64 / len_by_space as f64 
                + (j as f64/ notes_len as f64 ) * compound_unit;

                if x.contains("~") {
                    // sidechains.push(x.to_string());
                    self.sidechain_lib.insert(x.to_string(), self.sidechain_id);
                    self.sidechain_id += 1;
                }

                if x != &"_" {
                    events.push((relative_time, x.to_string()))
                }
            }
        }
        Self { events, ..self}
    }

    pub fn build(self) -> GlicolNodeData {
        mono_node!(self)
    }

}

#[macro_export]
macro_rules! seq {
    ({$($para: ident: $data:expr),*}) => {
         (
            Sequencer::new()$(.$para($data))*.build()
        )
    }
}

/// The Sequencer is probably the most complicated node in Glicol
/// The inputs can be clock, speed or many sidechains
impl Node<128> for Sequencer {
    fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {

        let num_sidechain = self.sidechain_lib.len();
        // let mut has_speed_input = false;

        match inputs.len() as i8 - num_sidechain as i8 {
            0 => { // standalone mode, no clock, no speed, just sidechain
                
            },
            1 => {
                //  clock or speed
                // speed input is set as [ f32, 0.0, 0.0 ... ], so it's identical
                // NOTE! inputs are in reverse order
        
                let m = inputs.len()-1;
                if inputs[m].buffers()[0][0] % 128. == 0. && inputs[m].buffers()[0][1] == 0. {
                    // this is very likely to be a clock
                    self.step = inputs[m].buffers()[0][0] as usize;

                } else if inputs[m].buffers()[0][0] % 128. > 0. && inputs[m].buffers()[0][1] == 0. {
                    // this can be a speed
                    self.speed = inputs[m].buffers()[0][0];
                } else {
                    // this is viewed as a sidechain note
                    return ()
                }
            },
            2 => {
                // with both speed and clock
                self.step = inputs[inputs.len()-1].buffers()[0][0] as usize;
                self.speed = inputs[inputs.len()-2].buffers()[0][0];
                // has_speed_input = true;
            },
            _ => {return ()}
        };

        // let relative_time = event.0;
        // let relative_pitch = event.1; a ratio for midi 60 freq
        let bar_length = 240.0 / self.bpm as f64 * self.sr as f64 / self.speed as f64;
        for i in 0..128 {
            output[0][i] = 0.0;

            for event in &self.events {
                if (self.step % (bar_length as usize)) == ((event.0 * bar_length) as usize) {

                    let midi = match event.1.parse::<f32>() {
                        Ok(val) => val,
                        Err(_) => {
                            // let len = inputs.len();
                            // there are cases:
                            // - no speed input, but has several sidechains
                            // - one speed input, no sidechain,
                            // - one speed input. several sidechains

                            // let index = len - 2 - 
                            // self.sidechain_lib[&event.1] - has_speed_input as usize;
                            // println!("index {}", index);
                            if inputs.len() as i8 - 1 > self.sidechain_lib[&event.1] as i8 {
                                inputs[self.sidechain_lib[&event.1]].buffers()[0][i]
                            } else {
                                return ()
                            }                         
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
