use dasp_graph::{Buffer, Input, Node};
use pest::iterators::Pairs;
use dasp_signal::{self as signal, Signal};
use super::super::{Rule, NodeData, BoxedNodeSend};
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
    pub fn new(paras: &mut Pairs<Rule>) -> (NodeData<BoxedNodeSend>, Vec<String>) {
        // let mut paras = paras.next().unwrap().into_inner();
        // let v: Vec<f32> = 
        // println!(">{:?}<", v);
        // let id = getrandom(&mut [0u8; 32]).unwrap();
        // let mut buf = [0u8; 1];
        // let c = getrandom(&mut buf);
        // Ok(buf);
        // println!("{:?} {:?}", c, buf);
        // let rng = OsRng;
        let note_list = paras.as_str().split(" ").map(
            |x|x.parse::<f32>().unwrap()).collect();
        let sig = signal::noise(0);

        (NodeData::new1(BoxedNodeSend::new( Self {
            // rng: rng,
            sig: Box::new(sig),
            note_list: note_list
        })), vec![])
    }
}

impl Node for Choose {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        // let id = self.rng.gen_range(0, self.note_list.len());
        // let note = self.note_list[id];
        let mut id = ((self.sig.next() * 0.5 + 0.5) * self.note_list.len() as f64) as usize;
        if id == self.note_list.len() {id = 0};
        output[0].iter_mut().for_each(|s| *s = self.note_list[id]);
    }
}