use glicol::*;

fn main() {
    let mut g = Graph::new(8, 2);
    let c = g.add_node(Box::new(SinOsc::new(440., 44100)));
    let m = g.add_node(Box::new(Mul::new(0.5)));
    g.add_edge(c, m).unwrap();
    g.add_edge(m, g.destination).unwrap();
    for _ in 0..1000 {
        let buf = g.yield_next_buffer();
        println!("buf {:?}", buf);
    }
}
