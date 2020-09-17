extern crate find_folder;
extern crate quaverseries_rs;
use quaverseries_rs::Engine;

fn main () {
    let mut engine = Engine::new();
    let assets = find_folder::Search::ParentsThenKids(5, 5).for_folder("assets").unwrap();
    engine.code = std::fs::read_to_string(assets.join("code.quaver")).unwrap();
    engine.update = true;
    for _ in 0..100 {
        println!("{:?}", engine.gen_next_buf_64().len());
    }
}