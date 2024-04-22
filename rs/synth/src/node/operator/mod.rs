mod mul;
pub use mul::Mul;
mod add;
pub use add::*;

use hashbrown::HashMap;
use crate::{Input, Buffer};

fn apply_op<const N: usize>(
    inputs: &mut HashMap<usize, Input<N>>,
    input_order: &[usize],
    output: &mut [Buffer<N>],
    val: f32,
    op: impl Fn(f32, f32) -> f32 + Copy
) {
    let (out_left, out_right) = output.split_at_mut(1);
    match inputs.len() {
        1 => {
            let main_input = inputs.values_mut().next().unwrap().buffers();

            for (idx, (out_left, main_in)) in out_left[0].iter_mut()
                .zip(main_input[0].iter())
                .enumerate()
            {
                *out_left = op(*main_in, val);

                if let [out_right, ..] = out_right {
                    out_right[idx] = op(main_input.get(1).map_or(*main_in, |m| m[idx]), val);
                }
            }
        }
        2 => {
            let main_input = inputs[&input_order[0]].buffers();
            let ref_input = inputs[&input_order[1]].buffers();

            for (idx, ((out_left, ref_in), main_in)) in out_left[0].iter_mut()
                .zip(ref_input[0].iter())
                .zip(main_input[0].iter())
                .enumerate()
            {
                *out_left = op(*main_in, *ref_in);

                if let [out_right, ..] = out_right {
                    out_right[idx] = op(
                        main_input.get(1).map_or(*main_in, |m| m[idx]),
                        ref_input.get(1).map_or(*ref_in, |r| r[idx])
                    );
                };
            }
        }
        _ => {}
    }
}
