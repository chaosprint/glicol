use dasp_graph::{Buffer, Input, Node};
use pest::iterators::Pairs;
use super::super::{Rule, NodeData, BoxedNodeSend};

pub struct Sequencer {
    events: Vec<(f64, f64)>,
    speed: f32,
    pub step: usize,
    _has_mod: bool
}

impl Sequencer {
    pub fn new(paras: &mut Pairs<Rule>) -> (NodeData<BoxedNodeSend>, Vec<String>) {
        let mut events = Vec::<(f64, f64)>::new();
        
        let mut paras = paras.next().unwrap().into_inner();

        let seq = paras.next().unwrap();
        let mut compound_index = 0;
        let seq_by_space: Vec<pest::iterators::Pair<Rule>> = 
        seq.clone().into_inner().collect();

        for compound in seq.into_inner() {
            let mut shift = 0;
            // calculate the length of seq
            let compound_vec: Vec<pest::iterators::Pair<Rule>> = 
            compound.clone().into_inner().collect();

            for note in compound.into_inner() {
                if note.as_str().parse::<i32>().is_ok() {
                    let seq_shift = 1.0 / seq_by_space.len() as f64 * 
                    compound_index as f64;
                    
                    let note_shift = 1.0 / compound_vec.len() as f64 *
                    shift as f64 / seq_by_space.len() as f64;

                    let d = note.as_str().parse::<i32>().unwrap() as f64;
                    let relative_pitch = 2.0f64.powf((d - 60.0)/12.0);
                    let relative_time = seq_shift + note_shift;
                    events.push((relative_time, relative_pitch));
                }
                shift += 1;
            }
            compound_index += 1;
        }

        (NodeData::new1(BoxedNodeSend::new( Self {
            events: events,
            speed: 1.0,
            step: 0,
            _has_mod: false
        })), vec![])
    }
}

impl Node for Sequencer {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        
        if inputs.len() > 0 {
            self.speed = inputs[0].buffers()[0][0];
        }
        // self.onebarlength = ?
        // let relative_time = event.0;
        // let relative_pitch = event.1; a ratio for midi 60 freq
        let bar_length = 88200.0 / self.speed as f64;
        for i in 0..64 {
            output[0][i] = 0.0;     
            for event in &self.events {
                // default bpm 120 -> 1 bar lasts 2 second, hence 88200.0
                if (self.step % (bar_length as usize)) == ((event.0 * bar_length) as usize) {
                    output[0][i] += event.1 as f32;
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
    pub fn new(paras: &mut Pairs<Rule>) -> (NodeData<BoxedNodeSend>, Vec<String>) {

        let speed: String = paras.next().unwrap().as_str().to_string()
        .chars().filter(|c| !c.is_whitespace()).collect();

        let is_float = speed.parse::<f32>();

        if is_float.is_ok() {
            (NodeData::new1(BoxedNodeSend::new(Self {speed: is_float.unwrap(), has_mod: false})),
            vec![])
        } else {
            (NodeData::new1(BoxedNodeSend::new(Self {speed: 0.0, has_mod: true})),
            vec![speed])
        }
    }
}
impl Node for Speed {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {

        if self.has_mod {
            assert!(inputs.len() > 0);
            let mod_buf = &mut inputs[0].buffers();
            // let mod_buf = &mut inputs[1].buffers();
            for i in 0..64 {
                output[0][i] = mod_buf[0][i];
            }
        } else {
            assert_eq!(inputs.len(), 0);
            // output[0] = inputs[0].buffers()[0].clone();
            output[0].iter_mut().for_each(|s| *s = self.speed as f32);
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