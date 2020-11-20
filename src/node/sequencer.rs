use dasp_graph::{Buffer, Input, Node};
use pest::iterators::Pairs;
use super::super::{HashMap, Rule, NodeData, BoxedNodeSend, EngineError};

pub struct Sequencer {
    events: Vec<(f64, String)>,
    speed: f32,
    pub step: usize,
    sidechain_lib: HashMap<String, usize>
}

impl Sequencer {
    pub fn new(paras: &mut Pairs<Rule>)
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
                let relative_time = i as f64 / len_by_space as f64 + (j as f64/ notes_len as f64 ) * compound_unit;

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
            events: events,
            speed: 1.0,
            step: 0,
            sidechain_lib: sidechain_lib
        })), sidechains))
    }
}

impl Node for Sequencer {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        
        let mut has_speed_input = false;
        
        if inputs.len() > 0 {
            // speed input is set as [ f32, 0.0, 0.0 ... ], so it's identical
            // NOTE! inputs are in reverse order

            // println!("input0 {} {}", inputs[0].buffers()[0][0],inputs[0].buffers()[0][1]);
            // println!("input0 {}, input1 {}, input2 {}", inputs[0].buffers()[0][0], 
            // inputs[1].buffers()[0][0], inputs[2].buffers()[0][0]);
            // println!("input0 {}, input1 {}, input2 {}", inputs[0].buffers()[0][1], 
            // inputs[1].buffers()[0][1], inputs[2].buffers()[0][1]);
            let last = inputs.len() - 1;
            if (inputs[last].buffers()[0][0] > 0.0) & (inputs[last].buffers()[0][1] == 0.0) {
                self.speed = inputs[last].buffers()[0][0];
                has_speed_input = true;
            }
        }

        // println!("speed {}", self.speed);
        // let relative_time = event.0;
        // let relative_pitch = event.1; a ratio for midi 60 freq
        let bar_length = 88200.0 / self.speed as f64;
        for i in 0..64 {
            output[0][i] = 0.0;

            for event in &self.events {
                if (self.step % (bar_length as usize)) == ((event.0 * bar_length) as usize) {

                    let midi = match event.1.parse::<f32>() {
                        Ok(val) => val,
                        Err(_why) => {
                            let len = inputs.len();

                            // there are cases:
                            // - no speed input, but has several sidechains
                            // - one speed input, no sidechain,
                            // - one speed input. several sidechains

                            let index = len - 1 - 
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
            self.step += 1;
        }
    }
}

pub struct Speed {
    pub speed: f32,
    has_mod: bool
}

impl Speed {
    pub fn new(paras: &mut Pairs<Rule>) -> Result<
    (NodeData<BoxedNodeSend>, Vec<String>), EngineError> {

        let speed: String = paras.as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        let is_float = speed.parse::<f32>();

        if is_float.is_ok() {
            Ok((NodeData::new1(BoxedNodeSend::new(
                Self {speed: is_float?, has_mod: false})),
            vec![]))
        } else {
            Ok((NodeData::new1(BoxedNodeSend::new(
                Self {speed: 1.0, has_mod: true})),
            vec![speed]))
        }
    }
}
impl Node for Speed {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        if self.has_mod {
            assert!(inputs.len() > 0);
            let mod_buf = &mut inputs[0].buffers();
            // let mod_buf = &mut inputs[1].buffers();
            // for i in 0..64 {
            output[0][0] = mod_buf[0][0];
            // }
        } else {
            assert_eq!(inputs.len(), 0);
            // output[0] = inputs[0].buffers()[0].clone();
            output[0][0] = self.speed as f32;
            // output[0].iter_mut().for_each(|s| *s = self.speed as f32);
        }
        // if inputs.len() > 0 {
    }
}

// impl Node for Speed {
//     fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
//         for o in output {
//             o.iter_mut().for_each(|s| *s = self.speed);
//         }
//     }
// }