extern crate lcs_diff;
use self::lcs_diff::*;

pub fn lcs(old: &Vec<String>, new: &Vec<String>)
-> (Vec<(String, usize)>, Vec<(String, usize)>, Vec<String>) {
    
    let mut add = Vec::new();
    let mut rem = Vec::new();
    let mut del = Vec::new();
    for diff in lcs_diff::diff(&old, &new) {
        println!("\n\nold {:?}, new {:?}\n\n", &old, &new);
        match diff {
            DiffResult::Added(a) => {
                add.push((a.data.clone(), a.new_index.unwrap()));
                println!(
                    "+{} new index = {}",
                    a.data,
                    a.new_index.unwrap()
                );                       
            },
            DiffResult::Common(c) => {
                rem.push((c.data.clone(), c.new_index.unwrap()));
                println!(
                    "{} old index = {}, new index = {}",
                    c.data,
                    c.old_index.unwrap(),
                    c.new_index.unwrap()
                );
            },
            DiffResult::Removed(r) => {
                println!("to remove!!! {:?}", r);
                del.push(r.data.clone());
                // println!(
                //     "-{} old index = {}",
                //     r.data,
                //     r.old_index.unwrap()
                // );
            }
        }
    };
    println!("\n{:?}{:?}{:?}\n", add, rem, del);
    (add, rem, del)
}