pub use sin_osc::SinOsc;
mod sin_osc;
pub use saw_osc::SawOsc;
mod saw_osc;
pub use tri_osc::TriOsc;
mod tri_osc;
pub use squ_osc::SquOsc;
mod squ_osc;

use crate::{Buffer, Input};
use hashbrown::HashMap;

fn process_oscillation<const N: usize>(
    inputs: &mut HashMap<usize, Input<N>>,
    input_order: &mut [usize],
    output: &mut [Buffer<N>],
    freq: f32,
    inc: &mut f32,
    mut osc: impl FnMut(&mut f32, f32)
) {
    match inputs.len() {
        0 => for out in &mut *output[0] {
            osc(out, freq);
        },
        1 => {
            let mod_input = match input_order {
                [] => &mut *inputs.values_mut().next().unwrap(),
                [ref first_input, ..] => &inputs[first_input],
            };

            for (out, mod_buf) in output[0].iter_mut().zip(mod_input.buffers()[0].iter()) {
                if *mod_buf != 0. {
                    *inc = *mod_buf;
                };

                osc(out, *inc);
            }
        }
        _ => {}
    }
}
