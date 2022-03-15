// use slice_diff_patch::*;
use lcs_diff::diff;

fn main() {
    let a = vec!["sin", "mul", "add"];
    let b = vec!["sin", "add", "lpf"];
    let result = diff(&a, &b);
    for res in result {
        println!("res {:?}", res);
    }
}