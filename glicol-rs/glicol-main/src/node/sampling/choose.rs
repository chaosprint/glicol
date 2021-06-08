use dasp_graph::{Buffer, Input, Node};
use dasp_signal::{self as signal, Signal};
use super::super::super::{GlicolNodeData, NodeData, mono_node, BoxedNodeSend};

pub struct Choose {
    sig: Box<dyn Signal<Frame=f64> + Send>,
    note_list: Vec::<f32>
}

impl Choose {
    pub fn new(note_list: Vec::<f32>) -> GlicolNodeData {
        mono_node! ( Self {sig: Box::new(signal::noise(42)), note_list} )
    }
}

#[macro_export]
macro_rules! choose {
    ($data: expr) => {
        Choose::new($data);
    };
}

impl Node<128> for Choose {
    fn process(&mut self, _inputs: &[Input<128>], output: &mut [Buffer<128>]) {
        let mut id = ((self.sig.next() * 0.5 + 0.5) * self.note_list.len() as f64) as usize;
        if id == self.note_list.len() {id = 0};
        output[0].iter_mut().for_each(|s| *s = self.note_list[id]);
    }
}