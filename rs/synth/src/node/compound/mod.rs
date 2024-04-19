mod bd;
pub use bd::*;
mod hh;
pub use hh::*;
mod sn;
pub use sn::*;
mod sawsynth;
pub use sawsynth::*;
mod squsynth;
pub use squsynth::*;
mod trisynth;
pub use trisynth::*;

use crate::{AudioContext, Buffer, Input};
use hashbrown::HashMap;
use petgraph::graph::NodeIndex;

fn process_compound<const N: usize>(
    inputs: &mut HashMap<usize, Input<N>>,
    input_order: &[usize],
    input: NodeIndex<u32>,
    context: &mut AudioContext<N>,
    output: &mut [Buffer<N>]
) {
    if inputs.len() == 1 {
        let main_input = inputs[&input_order[0]].buffers();
        context.graph[input].buffers[0] = main_input[0].clone();
        // self.context.graph[self.input].buffers[1] = main_input[1].clone();
        let cout = context.next_block();

        output[0][..N].copy_from_slice(&cout[0][..N]);
        output[1][..N].copy_from_slice(&cout[1][..N]);
    }
}
