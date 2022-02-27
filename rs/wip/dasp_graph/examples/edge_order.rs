
fn main() {
    fn main() {

        let mut graph = GlicolGraph::<128>::with_capacity(1024, 1024);
        let mut processor = GlicolProcessor::with_capacity(1024);

        let dc = graph.add_node( ConstSig::new(42.));
        let mul = graph.add_node( Mul::new(0.5));
        let _edge = graph.add_edge(dc, mul, ());

        let dc2 = graph.add_node( ConstSig::new(0.3));
        let _edge2 = graph.add_edge(dc2, mul, ());
        
        // at this point mul node has two inputs,
        // which are related to two different edges
        // we can tell mul node which edge is use for main, and which is for sidechain
        // with this send msg

        graph.send_msg(mul, Msg::InputOrder(0, dc));
        graph.send_msg(mul, Msg::InputOrder(1, dc2));

        // however, sending msg is optional, because in each node
        // the inputs order is determined by the order of edge creation

        // in this example, without sendmsg, `edge` will be `input[0]` of `mul` node
        // `edge2` will be the inputs[1] of `mul` node

        processor.process(&mut graph, index);


        println!("result {:?}", graph[index].buffers);

        // graph[index].node.send_msg((0, "440.0"));

        // processor.process(&mut graph, index);

        // println!("result after send msg {:?}", graph[index].buffers);

    }
}