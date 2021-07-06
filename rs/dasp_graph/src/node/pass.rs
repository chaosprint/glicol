use crate::{Buffer, Input, Node};

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
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        let input = match inputs.get(0) {
            None => return,
            Some(input) => input,
        };
        for (out_buf, in_buf) in output.iter_mut().zip(input.buffers()) {
            out_buf.copy_from_slice(in_buf);
        }
    }
}
