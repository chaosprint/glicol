extern crate lcs_diff;
use self::lcs_diff::*;
use super::{EngineError};

pub fn midi_or_float(num: String) -> f32 {
    if num.contains(".") {
        num.parse::<f32>().unwrap()
    } else {
        let midi = num.parse::<f32>().unwrap();
        if midi == 0.0 {
            0.0
        } else {
            2.0f32.powf((midi - 69.0)/12.0) * 440.0
        }
    }
}

pub fn code_preprocess(a: &String) -> Result<String, EngineError> {
    let q: String = a.replace("\n", " \n");
    let v: Vec<&str> = q.split(" ").collect();
    // println!("{:?}", v);
    let mut b = "".to_string();
    let mut skip = false;
    for (i, c) in v.iter().enumerate() {
        if *c == "sin" || *c == "saw" || *c == "squ" {
            if v[i+1].parse::<f32>().is_ok() {
                b += "const ";
                b += v[i+1];
                b += " >> ";
                b += c;
                skip = true;
            } else {
                b += c;
                b += " ";
            }
            // println!("{:?} {:?}", i, c);
        } else {
            if skip {
                b += " 1 ";
                skip = false;
            } else {
                b += c;
                b += " ";
            }
        }
    };
    Ok(b)
}

pub fn lcs(old: &Vec<String>, new: &Vec<String>)
-> (Vec<(String, usize)>, Vec<(String, usize)>, Vec<String>) {
    
    let mut add = Vec::new();
    let mut rem = Vec::new();
    let mut del = Vec::new();
    for diff in lcs_diff::diff(&old, &new) {
        // println!("\n\nold {:?}, new {:?}\n\n", &old, &new);
        match diff {
            DiffResult::Added(a) => {
                add.push((a.data.clone(), a.new_index.unwrap()));
                // println!(
                //     "+{} new index = {}",
                //     a.data,
                //     a.new_index.unwrap()
                // );                       
            },
            DiffResult::Common(c) => {
                rem.push((c.data.clone(), c.new_index.unwrap()));
                // println!(
                //     "{} old index = {}, new index = {}",
                //     c.data,
                //     c.old_index.unwrap(),
                //     c.new_index.unwrap()
                // );
            },
            DiffResult::Removed(r) => {
                // println!("to remove!!! {:?}", r);
                del.push(r.data.clone());
                // println!(
                //     "-{} old index = {}",
                //     r.data,
                //     r.old_index.unwrap()
                // );
            }
        }
    };
    // println!("\n{:?}{:?}{:?}\n", add, rem, del);
    (add, rem, del)
}

// pub fn clamp(input: f32, min: f32, max: f32) -> f32 {
//     match input {
//         c if c < min => min,
//         c if c > max => max,
//         _ => input
//     }
// }