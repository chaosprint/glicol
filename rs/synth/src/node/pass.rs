use crate::{Buffer, Input, Node, Message};
use hashbrown::HashMap;

/// A simple node that passes an input directly to the output.
///
/// Works by mem-copying each buffer of the first input to each buffer of the output respectively.
///
/// This can be useful as an intermediary node when feeding the output of a node back into one of
/// its inputs. It can also be useful for discarding excess input channels by having a `Pass` with
/// less output buffers than its input.
#[derive(Clone, Debug, PartialEq)]
pub struct Pass;

impl<const N: usize> Node<N> for Pass {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        let input = match inputs.values().next() {
            None => return,
            Some(input) => input,
        };
        if input.buffers().len() == 1 && output.len() == 2 {
            output[0].copy_from_slice(&input.buffers()[0]);
            output[1].copy_from_slice(&input.buffers()[0]);
        } else {
            for (out_buf, in_buf) in output.iter_mut().zip(input.buffers()) {
                out_buf.copy_from_slice(in_buf);
            }
        }
    }
    fn send_msg(&mut self, _info: Message) {
        
    }
}
