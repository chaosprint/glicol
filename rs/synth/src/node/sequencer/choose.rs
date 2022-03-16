use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};
use dasp_signal::{self as signal, Signal};
use hashbrown::HashMap;

pub struct Choose {
    sig: Box<dyn Signal<Frame=f64> + Send>,
    note_list: Vec::<f32>,
    input_order: Vec<usize>
}

impl Choose {
    pub fn new(note_list: Vec::<f32>, seed: u64) -> Self {
        Self {
            sig: Box::new(signal::noise(seed)), note_list,
            input_order: Vec::new()
        }
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for Choose {
    fn process(&mut self, _inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // TODO: better picking algo?
        let mut id = ((self.sig.next() * 0.5 + 0.5) * self.note_list.len() as f64) as usize;
        if id == self.note_list.len() {id = 0};
        output[0].iter_mut().for_each(|s| *s = self.note_list[id]);
    }
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToNumberList(pos, list) => {
                match pos {
                    0 => {self.note_list = list},
                    _ => {}
                }
            },
            Message::Index(i) => {
                self.input_order.push(i)
            },
            Message::IndexOrder(pos, index) => {
                self.input_order.insert(pos, index)
            },
            Message::ResetOrder => {
                self.input_order.clear();
            },
            _ => {}
        }
    }
}