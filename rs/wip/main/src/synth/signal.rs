use glicol_synth::{NodeData, BoxedNodeSend, Processor, Buffer, Input, Node};

#[derive(Debug, Copy, Clone)]
pub struct ConstSig<const N:usize> { val: f32 }

impl<const N:usize> ConstSig<N> {
    pub fn new(val: f32) -> NodeData<BoxedNodeSend<N>, N> {
        // NodeData::new1( Self {val} )
        NodeData::new1( BoxedNodeSend::<N>::new( Self {val} ) )
    }
}

impl<const N:usize> Node<N> for ConstSig<N> {
    fn process(&mut self, _inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            output[0][i] = self.val;
        }
    }
    fn send_msg(&mut self, info: (u8, &str)) {
        if info.0 == 0 && info.1.parse::<f32>().is_ok() {
            self.val = info.1.parse::<f32>().unwrap();
        }
    }
}