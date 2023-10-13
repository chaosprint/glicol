use glicol::*;

fn main() {
    let mut g = Graph2::<8, 2>::new(1024, 1024);
    g.update_order();
    for _ in 0..10 {
        let b = g.yield_next_buffer().unwrap();
        println!("out buffer {:?}", b);
    }
}
