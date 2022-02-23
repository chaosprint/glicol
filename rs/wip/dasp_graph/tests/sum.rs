#![cfg(all(feature = "node-boxed", feature = "node-sum"))]

use dasp_graph::{node, Buffer, Input, Node, NodeData};

type BoxedNode = dasp_graph::BoxedNode<128>;

// A simple source node that just writes `0.1` to the output. We'll use this to test the sum node.
fn src_node(_inputs: &[Input<128>], output: &mut [Buffer<128>]) {
    for o in output {
        o.iter_mut().for_each(|s| *s = 0.1);
    }
}

#[test]
fn test_sum() {
    // The type of graph to use for this test.
    type Graph = petgraph::Graph<NodeData<BoxedNode, 128>, (), petgraph::Directed, u32>;
    type Processor = dasp_graph::Processor<Graph, 128>;

    // Create a graph and a processor.
    let max_nodes = 6;
    let max_edges = 5;
    let mut g = Graph::with_capacity(max_nodes, max_edges);
    let mut p = Processor::with_capacity(max_nodes);

    // Create and add the nodes to the graph.
    let src_node_ptr = src_node as fn(&[Input<128>], &mut [Buffer<128>]);
    let src_a = g.add_node(NodeData::new1(BoxedNode::new(src_node_ptr)));
    let src_b = g.add_node(NodeData::new1(BoxedNode::new(src_node_ptr)));
    let sum = g.add_node(NodeData::new1(BoxedNode::new(node::Sum)));

    // Plug the source nodes into the sum node.
    g.add_edge(src_a, sum, ());
    g.add_edge(src_b, sum, ());

    // Process the graph from the sum node.
    p.process(&mut g, sum);

    // Check that `sum` actually contains the sum.
    let expected = Buffer::from([0.2; 128]);
    assert_eq!(&g[sum].buffers[..], &[expected][..]);

    // Plug in some more sources.
    let src_c = g.add_node(NodeData::new1(BoxedNode::new(src_node_ptr)));
    let src_d = g.add_node(NodeData::new1(BoxedNode::new(src_node_ptr)));
    let src_e = g.add_node(NodeData::new1(BoxedNode::new(src_node_ptr)));
    g.add_edge(src_c, sum, ());
    g.add_edge(src_d, sum, ());
    g.add_edge(src_e, sum, ());

    // Check that the result is consistent.
    p.process(&mut g, sum);
    let expected = Buffer::from([0.5; 128]);
    assert_eq!(&g[sum].buffers[..], &[expected][..]);
}

#[test]
fn test_sum2() {
    // The type of graph to use for this test.
    type Graph = petgraph::Graph<NodeData<BoxedNode, 128>, (), petgraph::Directed, u32>;
    type Processor = dasp_graph::Processor<Graph, 128>;

    // Create a graph and a processor.
    let mut g = Graph::new();
    let mut p = Processor::with_capacity(g.node_count());

    // Create a small tree where we first sum a and b, then sum the result with c.
    // This time, using two buffers (channels) per node.
    let src_node_ptr = src_node as fn(&[Input<128>], &mut [Buffer<128>]);
    let src_a = g.add_node(NodeData::new2(BoxedNode::new(src_node_ptr)));
    let src_b = g.add_node(NodeData::new2(BoxedNode::new(src_node_ptr)));
    let src_c = g.add_node(NodeData::new2(BoxedNode::new(src_node_ptr)));
    let sum_a_b = g.add_node(NodeData::new2(BoxedNode::new(node::Sum)));
    let sum_ab_c = g.add_node(NodeData::new2(BoxedNode::new(node::Sum)));
    g.add_edge(src_a, sum_a_b, ());
    g.add_edge(src_b, sum_a_b, ());
    g.add_edge(sum_a_b, sum_ab_c, ());
    g.add_edge(src_c, sum_ab_c, ());

    // Process the graph.
    p.process(&mut g, sum_ab_c);

    // sum_a_b should be 0.2.
    let expected = vec![Buffer::from([0.2; 128]); 2];
    assert_eq!(&g[sum_a_b].buffers[..], &expected[..]);
    // sum_ab_c should be 0.3.
    let expected = vec![Buffer::from([0.3; 128]); 2];
    assert_eq!(&g[sum_ab_c].buffers[..], &expected[..]);
}

#[test]
fn test_sum_unboxed() {
    // Prove to ourselves we also support unboxed node types with a custom node type.
    enum TestNode {
        SourceFnPtr(fn(&[Input<128>], &mut [Buffer<128>])),
        Sum(node::Sum),
    }

    impl Node<128> for TestNode {
        fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
            match *self {
                TestNode::SourceFnPtr(ref mut f) => (*f)(inputs, output),
                TestNode::Sum(ref mut sum) => sum.process(inputs, output),
            }
        }
    }

    // The type of graph to use for this test.
    type Graph = petgraph::Graph<NodeData<TestNode, 128>, (), petgraph::Directed, u32>;
    type Processor = dasp_graph::Processor<Graph, 128>;

    // Create a graph and a processor.
    let mut g = Graph::new();
    let mut p = Processor::with_capacity(g.node_count());

    // Add two source nodes and a sum node.
    let src_node_ptr = src_node as _;
    let src_a = g.add_node(NodeData::new1(TestNode::SourceFnPtr(src_node_ptr)));
    let src_b = g.add_node(NodeData::new1(TestNode::SourceFnPtr(src_node_ptr)));
    let sum = g.add_node(NodeData::new1(TestNode::Sum(node::Sum)));

    // Plug the source nodes into the sum node.
    g.add_edge(src_a, sum, ());
    g.add_edge(src_b, sum, ());

    // Process the graph from the sum node.
    p.process(&mut g, sum);

    // Check that `sum` actually contains the sum.
    let expected = Buffer::from([0.2; 128]);
    assert_eq!(&g[sum].buffers[..], &[expected][..]);
}
