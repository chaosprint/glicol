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

// modified: add const generics

use core::fmt;
use core::ops::{Deref, DerefMut};

/// The fixed-size buffer used for processing the graph.
#[derive(Clone)]
pub struct Buffer<const N: usize> {
    data: [f32; N],
}

impl<const N: usize> Buffer<N> {
    pub const LEN: usize = N;
    /// A silent **Buffer**.
    pub const SILENT: Self = Buffer { data: [0.0; N] };

    /// Short-hand for writing silence to the whole buffer.
    pub fn silence(&mut self) {
        self.data.copy_from_slice(&Self::SILENT)
    }
}

impl<const N: usize> Default for Buffer<N> {
    fn default() -> Self {
        Self::SILENT
    }
}

impl<const N: usize> From<[f32; N]> for Buffer<N> {
    fn from(data: [f32; N]) -> Self {
        Buffer { data }
    }
}

impl<const N: usize> fmt::Debug for Buffer<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.data[..], f)
    }
}

impl<const N: usize> PartialEq for Buffer<N> {
    fn eq(&self, other: &Self) -> bool {
        &self[..] == &other[..]
    }
}

impl<const N: usize> Deref for Buffer<N> {
    type Target = [f32];
    fn deref(&self) -> &Self::Target {
        &self.data[..]
    }
}

impl<const N: usize> DerefMut for Buffer<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[..]
    }
}
