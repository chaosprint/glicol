use crate::{Buffer, Input, Node};

#[derive(Debug, Copy, Clone)]
struct Add<const N:usize> { val: f32 }

impl<const N:usize> Add<N> {
    pub fn new(val: f32) -> NodeData<BoxedNodeSend<N>, N> {
        NodeData::new1( BoxedNodeSend::<N>::new( Self { val } ) )
    }
}

impl<const N:usize> Node<N> for Add<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        for i in 0..N {
            output[0][i] = inputs[0].buffers()[0][i] + self.val;
        }
    }
    fn send_msg(&mut self, info: (u8, &str)) {
        if info.0 == 0 {
            self.val = info.1.parse::<f32>().unwrap();
        }
    }
}
