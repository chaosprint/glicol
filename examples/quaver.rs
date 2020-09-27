extern crate find_folder;
extern crate quaverseries_rs;
use quaverseries_rs::Engine;

fn main () {
    let mut engine = Engine::new();
    let assets = find_folder::Search::ParentsThenKids(5, 5).for_folder("assets").unwrap();
    engine.set_code(std::fs::read_to_string(assets.join("filter.quaver")).unwrap());
    engine.update();
    for _ in 0..(88200.0/128.0) as usize {
        engine.gen_next_buf_128();
    }
    let out = engine.gen_next_buf_128();
    for i in 0..128 {
        print!("{:?}, ", out[i]);
    }
}