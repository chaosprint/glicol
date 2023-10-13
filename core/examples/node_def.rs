use glicol::*;

fn main() {
    let mut g = Graph::new(8, 2);
    let processor = NodeDef::new("[0.42, 0.48, 0.42, 0.48, 0.42, 0.48, 0.42, 0.48]");
    let c = g.add_node(Box::new(processor));
    let m = g.add_node(Box::new(Mul::new(0.5)));
    g.add_edge(c, m).unwrap();
    g.add_edge(m, g.destination).unwrap();
    for _ in 0..1000 {
        let buf = g.yield_next_buffer();
        println!("buf {:?}", buf);
    }
}
