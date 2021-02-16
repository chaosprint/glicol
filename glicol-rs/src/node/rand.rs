use dasp_graph::{Buffer, Input, Node};
use pest::iterators::Pairs;
use dasp_signal::{self as signal, Signal};
use super::super::{Rule, NodeData, NodeResult, 
    BoxedNodeSend, EngineError};
// use rand_core::{RngCore, OsRng};
// use rand::Rng;
// use rand::rngs::OsRng;
// use rand::seq::SliceRandom;
// use getrandom::getrandom;

pub struct Choose {
    // rng: OsRng,
    sig: Box<dyn Signal<Frame=f64> + Send>,
    note_list: Vec::<f32>
}

impl Choose {
    pub fn new(paras: &mut Pairs<Rule>) -> NodeResult {
        // let mut paras = paras.next().unwrap().into_inner();
        // let v: Vec<f32> = 
        // println!(">{:?}<", v);
        // let id = getrandom(&mut [0u8; 32]).unwrap();
        // let mut buf = [0u8; 1];
        // let c = getrandom(&mut buf);
        // Ok(buf);
        // println!("{:?} {:?}", c, buf);
        // let rng = OsRng;
        let split: Vec<&str> = paras.as_str().split(" ").collect();
        let p = paras.next().unwrap();
        let pos = (p.as_span().start(), p.as_span().end());
        let mut note_list = Vec::<f32>::new();
        for note in split {
            match note.parse::<f32>() {
                Ok(v) => note_list.push(v),
                Err(_) => return Err(EngineError::ParameterError(pos))
            }
        }
        // = split.iter().map(|x|x.parse::<f32>().unwrap()).collect();
        let sig = signal::noise(0);

        Ok((NodeData::new1(BoxedNodeSend::new( Self {
            // rng: rng,
            sig: Box::new(sig),
            note_list: note_list
        })), vec![]))
    }
}

impl Node<128> for Choose {
    fn process(&mut self, _inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        // let id = self.rng.gen_range(0, self.note_list.len());
        // let note = self.note_list[id];
        let mut id = ((self.sig.next() * 0.5 + 0.5) * self.note_list.len() as f64) as usize;
        if id == self.note_list.len() {id = 0};
        output[0].iter_mut().for_each(|s| *s = self.note_list[id]);
    }
}