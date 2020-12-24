use dasp_graph::{Buffer, Input, Node};
use pest::iterators::Pairs;
use super::super::{HashMap, Rule, NodeData, BoxedNodeSend, EngineError, handle_params};

// pub struct

pub struct Sequencer {
    events: Vec<(f64, String)>,
    speed: f32,
    bpm: f32,
    sr: f32,
    pub step: usize,
    sidechain_lib: HashMap<String, usize>
}

impl Sequencer {
    pub fn new(paras: &mut Pairs<Rule>, sr: f32, bpm: f32)
        -> Result<(NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

        let mut events = Vec::<(f64, String)>::new();
        let mut sidechains = Vec::<String>::new();
        let mut sidechain_id = 0;
        let mut sidechain_lib = HashMap::<String, usize>::new();

        let split: Vec<&str> = paras.as_str().split(" ").collect();

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
                    sidechains.push(x.to_string());
                    sidechain_lib.insert(x.to_string(), sidechain_id);
                    sidechain_id += 1;
                }

                if x != &"_" {
                    events.push((relative_time, x.to_string()))
                }
            }
        }
        // println!("{:?}", split);
        // println!("sidechains {:?}", sidechains);
        // println!("events {:?}", events);

        Ok((NodeData::new1(BoxedNodeSend::new( Self {
            sr,
            bpm,
            events: events,
            speed: 1.0,
            step: 0,
            sidechain_lib: sidechain_lib
        })), sidechains))
    }
}

impl Node for Sequencer {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        let mut step = inputs[inputs.len()-1].buffers()[0][0] as usize;
        
        let mut has_speed_input = false;
        
        if inputs.len() > 1 {
            // except for the clock, the only optional input should be the speed node
            
            // speed input is set as [ f32, 0.0, 0.0 ... ], so it's identical
            // NOTE! inputs are in reverse order

            let last = inputs.len() - 2; // -1 is the clock
            if (inputs[last].buffers()[0][0] > 0.0) &&
            (inputs[last].buffers()[0][1] == 0.0) { // make sure it is speed input
                self.speed = inputs[last].buffers()[0][0];
                has_speed_input = true;
            }
        }

        // println!("speed {}", self.speed);
        // let relative_time = event.0;
        // let relative_pitch = event.1; a ratio for midi 60 freq
        let bar_length = 240.0 / self.bpm as f64 * self.sr as f64 / self.speed as f64;
        for i in 0..64 {
            output[0][i] = 0.0;

            for event in &self.events {
                if (step % (bar_length as usize)) == ((event.0 * bar_length) as usize) {

                    let midi = match event.1.parse::<f32>() {
                        Ok(val) => val,
                        Err(_) => {
                            let len = inputs.len();

                            // there are cases:
                            // - no speed input, but has several sidechains
                            // - one speed input, no sidechain,
                            // - one speed input. several sidechains

                            let index = len - 2 - 
                            self.sidechain_lib[&event.1] - has_speed_input as usize;
                            // println!("index {}", index);
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
            step += 1;
        }
    }
}

pub struct Speed {
    speed: f32,
    sidechain_ids: Vec<u8>
}

impl Speed {
    handle_params!({
        speed: 1.0
    });
}

impl Node for Speed {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        if self.sidechain_ids.len() > 0 {
            let mod_buf = &mut inputs[0].buffers();
            output[0][0] = mod_buf[0][0];
        } else {
            output[0][0] = self.speed as f32;
        }
    }
}