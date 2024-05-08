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

#![doc = include_str!("../README.md")]
mod context;

pub use context::*;

mod graph;
use glicol_parser::{ToInnerOwned, nodes::{NumberOrRef, TimeList}};
pub use graph::*;

mod node;
pub use node::{
    compound, delay, effect, envelope, filter, operator, oscillator, sequencer, signal, synth,
};
pub use node::{Input, Node};
// pub use node::*; // TODO: Do not expose every struct here

mod buffer;
pub use buffer::Buffer;

#[cfg(feature = "node-sampling")]
pub use node::sampling;

#[cfg(feature = "node-dynamic")]
pub use node::dynamic;

#[cfg(feature = "node-boxed")]
pub use node::{BoxedNode, BoxedNodeSend};

#[cfg(feature = "node-sum")]
pub use node::{Sum, Sum2};

#[cfg(feature = "node-pass")]
pub use node::Pass;

use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    SetToNumber(u8, f32),
    SetToNumberList(u8, Vec<f32>),
    SetToSymbol(u8, String),
    SetToSamples(u8, (&'static [f32], usize, usize)),
    SetSamplePattern(
        Vec<(String, f32)>,
        f32,
        HashMap<String, (&'static [f32], usize, usize)>,
    ),
    SetPattern(Vec<(f32, f32)>, f32),
    SetToSeq(u8, Vec<(f32, NumberOrRef<String>)>),
    SetRefOrder(HashMap<String, usize>),
    SetBPM(f32),
    SetSampleRate(usize),
    MainInput(petgraph::graph::NodeIndex),
    SidechainInput(petgraph::graph::NodeIndex),
    Index(usize),
    IndexOrder(usize, usize),
    ResetOrder,
    SetParam(u8, GlicolPara<String>),
    SetToBool(u8, bool),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum GlicolPara<S>
where
    S: AsRef<str>
{
    Number(f32),
    Bool(bool),
    NumberList(Vec<f32>),
    Reference(S),
    SampleSymbol(S), // symbol is for sample only
    Symbol(S),
    Sequence(Vec<(f32, NumberOrRef<S>)>),
    Pattern(Vec<(Self, f32)>, f32),
    Event(Vec<(Self, f32)>),
    Points(Vec<(TimeList, f32)>),
    Bar(f32),
    Second(f32),
    Millisecond(f32),
}

impl<S> ToInnerOwned for GlicolPara<&S>
where
    S: AsRef<str> + ToOwned + ?Sized,
    <S as ToOwned>::Owned: AsRef<str>,
{
    type Owned = GlicolPara<<S as ToOwned>::Owned>;
    fn to_inner_owned(&self) -> Self::Owned {
        match self {
            Self::Number(num) => GlicolPara::Number(*num),
            Self::Bool(bool) => GlicolPara::Bool(*bool),
            Self::NumberList(vec) => GlicolPara::NumberList(vec.to_vec()),
            Self::Reference(s) => GlicolPara::Reference((*s).to_owned()),
            Self::SampleSymbol(s) => GlicolPara::SampleSymbol((*s).to_owned()),
            Self::Symbol(s) => GlicolPara::Symbol((*s).to_owned()),
            Self::Sequence(seq) => GlicolPara::Sequence(seq.to_inner_owned()),
            Self::Pattern(vec, num) => GlicolPara::Pattern(vec.to_inner_owned(), *num),
            Self::Event(vec) => GlicolPara::Event(vec.to_inner_owned()),
            Self::Points(vec) => GlicolPara::Points(vec.to_inner_owned()),
            Self::Bar(num) => GlicolPara::Bar(*num),
            Self::Second(num) => GlicolPara::Second(*num),
            Self::Millisecond(num) => GlicolPara::Millisecond(*num)
        }
    }
}
