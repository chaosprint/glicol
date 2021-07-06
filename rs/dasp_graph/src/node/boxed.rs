use crate::{Buffer, Input, Node};
use core::fmt;
use core::ops::{Deref, DerefMut};

/// A wrapper around a `Box<dyn Node>`.
///
/// Provides the necessary `Sized` implementation to allow for compatibility with the graph process
/// function.
pub struct BoxedNode<const N: usize>(pub Box<dyn Node<N>>);

/// A wrapper around a `Box<dyn Node>`.
///
/// Provides the necessary `Sized` implementation to allow for compatibility with the graph process
/// function.
///
/// Useful when the ability to send nodes from one thread to another is required. E.g. this is
/// common when initialising nodes or the audio graph itself on one thread before sending them to
/// the audio thread.
pub struct BoxedNodeSend<const N: usize>(pub Box<dyn Node<N> + Send>);

impl<const N: usize> BoxedNode<N> {
    /// Create a new `BoxedNode` around the given `node`.
    ///
    /// This is short-hand for `BoxedNode::from(Box::new(node))`.
    pub fn new<T>(node: T) -> Self
    where
        T: 'static + Node<N>,
    {
        Self::from(Box::new(node))
    }
}

impl<const N: usize> BoxedNodeSend<N> {
    /// Create a new `BoxedNode` around the given `node`.
    ///
    /// This is short-hand for `BoxedNode::from(Box::new(node))`.
    pub fn new<T>(node: T) -> Self
    where
        T: 'static + Node<N> + Send,
    {
        Self::from(Box::new(node))
    }
}

impl<const N: usize> Node<N> for BoxedNode<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        self.0.process(inputs, output)
    }
}

impl<const N: usize> Node<N> for BoxedNodeSend<N> {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        self.0.process(inputs, output)
    }
}

impl<T, const N: usize> From<Box<T>> for BoxedNode<N>
where
    T: 'static + Node<N>,
{
    fn from(n: Box<T>) -> Self {
        BoxedNode(n as Box<dyn Node<N>>)
    }
}

impl<T, const N: usize> From<Box<T>> for BoxedNodeSend<N>
where
    T: 'static + Node<N> + Send,
{
    fn from(n: Box<T>) -> Self {
        BoxedNodeSend(n as Box<dyn Node<N> + Send>)
    }
}

impl<const N: usize> Into<Box<dyn Node<N>>> for BoxedNode<N> {
    fn into(self) -> Box<dyn Node<N>> {
        self.0
    }
}

impl<const N: usize> Into<Box<dyn Node<N> + Send>> for BoxedNodeSend<N> {
    fn into(self) -> Box<dyn Node<N> + Send> {
        self.0
    }
}

impl<const N: usize> fmt::Debug for BoxedNode<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoxedNode").finish()
    }
}

impl<const N: usize> fmt::Debug for BoxedNodeSend<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoxedNodeSend").finish()
    }
}

impl<const N: usize> Deref for BoxedNode<N> {
    type Target = Box<dyn Node<N>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Deref for BoxedNodeSend<N> {
    type Target = Box<dyn Node<N> + Send>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for BoxedNode<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> DerefMut for BoxedNodeSend<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
