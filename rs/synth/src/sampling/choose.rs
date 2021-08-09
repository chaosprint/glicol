use dasp_graph::{Buffer, Input, Node};
use dasp_signal::{self as signal, Signal};
use super::super::{GlicolNodeData, NodeData, mono_node, BoxedNodeSend};

pub struct Choose<const N:usize> {
    sig: Box<dyn Signal<Frame=f64> + Send>,
    note_list: Vec::<f32>
}

impl<const N:usize> Choose<N> {
    pub fn new(note_list: Vec::<f32>) -> GlicolNodeData<N> {
        mono_node! ( N, Self {sig: Box::new(signal::noise(42)), note_list} )
    }
}

#[macro_export]
macro_rules! choose {
    ($data: expr) => {
        Choose::new($data);
    };
}

impl<const N:usize> Node<N> for Choose<N> {
    fn process(&mut self, _inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        let mut id = ((self.sig.next() * 0.5 + 0.5) * self.note_list.len() as f64) as usize;
        if id == self.note_list.len() {id = 0};
        output[0].iter_mut().for_each(|s| *s = self.note_list[id]);
    }
}