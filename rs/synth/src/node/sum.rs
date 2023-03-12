// The MIT License (MIT)

// Copyright (c) 2016 RustAudio Developers

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::{Buffer, Input, Node, Message};
use hashbrown::HashMap;

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

#[derive(Clone, Debug, PartialEq)]
pub struct Sum2;

impl<const N: usize> Node<N> for Sum {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // Fill the output with silence.
        for out_buffer in output.iter_mut() {
            out_buffer.silence();
        }
        // Sum the inputs onto the output.
        for (channel, out_buffer) in output.iter_mut().enumerate() {
            for input in inputs.values() {
                let in_buffers = input.buffers();
                if let Some(in_buffer) = in_buffers.get(channel) {
                    dasp_slice::add_in_place(out_buffer, in_buffer);
                }
            }
        }
        // println!("{:?}", output);
    }
    fn send_msg(&mut self, _info: Message) {
        
    }
}



impl<const N: usize> Node<N> for Sum2 {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // Fill the output with silence.
        for out_buffer in output.iter_mut() {
            out_buffer.silence();
        }
        // Sum the inputs onto the output.
        for (channel, out_buffer) in output.iter_mut().enumerate() {
            for input in inputs.values() {
                let in_buffers = input.buffers();
                match in_buffers.get(channel) {
                    Some(in_buffer) => {
                        dasp_slice::add_in_place(out_buffer, in_buffer);
                    },
                    None => {
                        dasp_slice::add_in_place(out_buffer, &in_buffers[0]);
                    }
                };
            }
        }
    }
    fn send_msg(&mut self, _info: Message) {
        
    }
}

impl<const N: usize> Node<N> for SumBuffers {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        // Get the first output buffer.
        let mut out_buffers = output.iter_mut();
        let out_buffer_first = match out_buffers.next() {
            None => return,
            Some(buffer) => buffer,
        };
        // Fill it with silence.
        out_buffer_first.silence();
        // Sum all input buffers onto the first output buffer.
        for input in inputs.values() {
            for in_buffer in input.buffers() {
                dasp_slice::add_in_place(out_buffer_first, in_buffer);
            }
        }
        // Write the first output buffer to the rest.
        for out_buffer in out_buffers {
            out_buffer.copy_from_slice(out_buffer_first);
        }
    }
    fn send_msg(&mut self, _info: Message) {
        
    }
}
