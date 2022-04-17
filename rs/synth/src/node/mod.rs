use crate::buffer::Buffer;
use hashbrown::HashMap;

#[cfg(feature = "node-boxed")]
mod boxed; pub use boxed::*;
#[cfg(feature = "node-pass")]
mod pass; pub use pass::*;
#[cfg(feature = "node-sum")]
mod sum; pub use sum::*;

pub mod oscillator; pub use oscillator::*;
pub mod operator; pub use operator::*;
pub mod signal; pub use signal::*;
pub mod filter; pub use filter::*;

pub mod sequencer; pub use sequencer::*;
pub mod delay; pub use delay::*;
pub mod envelope; pub use envelope::*;
pub mod effect; pub use effect::*;
pub mod compound; pub use compound::*;

pub mod synth; pub use synth::*;

#[cfg(feature = "use-samples")]
pub mod sampling;

#[cfg(feature = "use-samples")]
pub use sampling::*;

#[cfg(feature = "use-meta")]
pub mod dynamic;

#[cfg(feature = "use-meta")]
pub use dynamic::*;

pub trait Node<const N: usize> {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]);
    fn send_msg(&mut self, info: crate::Message);
}

/// An important part of the `Node` trait; each `Input` contains the relevant node id as `usize`
pub struct Input<const N: usize> {
    buffers_ptr: *const Buffer<N>,
    buffers_len: usize,
    pub node_id: usize
}

impl<const N: usize> Input<N> {
    // Constructor solely for use within the graph `process` function.
    pub(crate) fn new(slice: &[Buffer<N>], node_id: usize) -> Self {
        let buffers_ptr = slice.as_ptr();
        let buffers_len = slice.len();
        Input {
            buffers_ptr,
            buffers_len,
            node_id
        }
    }

    /// A reference to the buffers of the input node.
    pub fn buffers(&self) -> &[Buffer<N>] {
        // As we know that an `Input` can only be constructed during a call to the graph `process`
        // function, we can be sure that our slice is still valid as long as the input itself is
        // alive.
        unsafe { std::slice::from_raw_parts(self.buffers_ptr, self.buffers_len) }
    }
}

// Inputs can only be created by the `dasp_graph::process` implementation and only ever live as
// long as the lifetime of the call to the function. Thus, it's safe to implement this so that
// `Send` closures can be stored within the graph and sent between threads.
unsafe impl<const N: usize> Send for Input<N> {}

impl<const N: usize> core::fmt::Debug for Input<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(self.buffers(), f)
    }
}

impl<'a, T, const N: usize> Node<N> for &'a mut T
where
    T: Node<N>,
{
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        (**self).process(inputs, output)
    }
    fn send_msg(&mut self, info: crate::Message) {
        (**self).send_msg(info)
    }
}

impl<T, const N: usize> Node<N> for Box<T>
where
    T: Node<N>,
{
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        (**self).process(inputs, output)
    }
    fn send_msg(&mut self, _info: crate::Message) {
    }
}

impl<const N: usize> Node<N> for dyn Fn(&HashMap<usize, Input<N>>, &mut [Buffer<N>]) {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        (*self)(inputs, output)
    }
    fn send_msg(&mut self, _info: crate::Message) {
    }
}

impl<const N: usize> Node<N> for dyn FnMut(&HashMap<usize, Input<N>>, &mut [Buffer<N>]) {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        (*self)(inputs, output)
    }
    
    fn send_msg(&mut self, _info: crate::Message) {
    }
}

impl<const N: usize> Node<N> for fn(&HashMap<usize, Input<N>>, &mut [Buffer<N>]) {
    fn process(&mut self, inputs: &mut HashMap<usize, Input<N>>, output: &mut [Buffer<N>]) {
        (*self)(inputs, output)
    }
    fn send_msg(&mut self, _info: crate::Message) {
    }
}