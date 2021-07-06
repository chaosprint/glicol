use crate::{Buffer, Input, Node};

/// A stateless node that sums each of the inputs onto the output.
///
/// Assumes that the number of buffers per input is equal to the number of output buffers.
#[derive(Clone, Debug, PartialEq)]
pub struct Sum;

/// A stateless node that sums all of the buffers of all of the inputs onto each of the output
/// buffers.
///
/// E.g. Given two inputs with three buffers each, all 6 input buffers will be summed onto the
/// first output buffer. If there is more than one output buffer, the result is copied to the
/// remaining output buffers.
///
/// After a call to `Node::process`, each of the output buffers will always have the same contents.
///
/// Common use cases:
///
/// - Summing multiple input channels down to a single output channel.
/// - Writing a single input channel to multiple output channels.
#[derive(Clone, Debug, PartialEq)]
pub struct SumBuffers;

impl<const N: usize> Node<N> for Sum {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        // Fill the output with silence.
        for out_buffer in output.iter_mut() {
            out_buffer.silence();
        }
        // Sum the inputs onto the output.
        for (channel, out_buffer) in output.iter_mut().enumerate() {
            for input in inputs {
                let in_buffers = input.buffers();
                if let Some(in_buffer) = in_buffers.get(channel) {
                    dasp_slice::add_in_place(out_buffer, in_buffer);
                }
            }
        }
    }
}

impl<const N: usize> Node<N> for SumBuffers {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        // Get the first output buffer.
        let mut out_buffers = output.iter_mut();
        let out_buffer_first = match out_buffers.next() {
            None => return,
            Some(buffer) => buffer,
        };
        // Fill it with silence.
        out_buffer_first.silence();
        // Sum all input buffers onto the first output buffer.
        for input in inputs {
            for in_buffer in input.buffers() {
                dasp_slice::add_in_place(out_buffer_first, in_buffer);
            }
        }
        // Write the first output buffer to the rest.
        for out_buffer in out_buffers {
            out_buffer.copy_from_slice(out_buffer_first);
        }
    }
}
