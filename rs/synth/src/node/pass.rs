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
