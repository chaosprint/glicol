use crate::{Buffer, Input, Node};
use dasp_ring_buffer as ring_buffer;

/// A delay node, where the delay duration for each channel is equal to the length of the inner
/// ring buffer associated with that channel.
///
/// Assumes that there is one input node, and that the number of input buffers, output buffers and
/// ring buffers all match.
#[derive(Clone, Debug, PartialEq)]
pub struct Delay<S>(pub Vec<ring_buffer::Fixed<S>>);

impl<S, const N: usize> Node<N> for Delay<S>
where
    S: ring_buffer::SliceMut<Element = f32>,
{
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        // Retrieve the single input, ignore any others.
        let input = match inputs.get(0) {
            Some(input) => input,
            None => return,
        };

        // Apply the delay across each channel.
        for ((ring_buf, in_buf), out_buf) in self.0.iter_mut().zip(input.buffers()).zip(output) {
            for (i, out) in out_buf.iter_mut().enumerate() {
                *out = ring_buf.push(in_buf[i]);
            }
        }
    }
}
