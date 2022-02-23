use slice_diff_patch::*;

fn main() {
    let a = vec![("sin", "440.0"), ("mul", "0.3")];
    let b = vec![("sin", "220.0"), ("mul", "0.3"), ("add", "0.5")];
    let diff = diff_diff(&a, &b);
    println!("diff diff {:?}", diff);
    
    let lcs = lcs_diff(&a, &b);
    println!("lcs_diff {:?}", lcs);

    let wu = wu_diff(&a, &b);
    println!("wu_diff {:?}", wu);
}