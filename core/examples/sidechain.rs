use glicol::*;

fn main() {
    let mut g = Graph::new(4, 2);
    let c = g.add_node(Box::new(Constant::new(42.)));
    let m = g.add_node(Box::new(Mul::new(0.5)));
    let side = g.add_node(Box::new(Constant::new(0.1)));
    g.add_edge(c, m).unwrap();
    g.add_edge(side, m).unwrap();
    g.add_edge(m, g.destination).unwrap();
    for item in &g.process_order {
        g.graph_nodes[*item].inspect();
    }
    for _ in 0..3 {
        let buf = g.yield_next_buffer();
        println!("buf {:?}", buf);
    }
}
