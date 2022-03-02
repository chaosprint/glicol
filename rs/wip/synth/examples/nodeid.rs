use glicol_synth::{AudioContextBuilder, Buffer, Input, Node, Message, ConstSig};

struct Powf {
    val: f32,
    main_input: usize,
    sidechain_input: usize,
}

impl Powf {
    pub fn new(val: f32) -> Self { Self {val, main_input:0, sidechain_input:0 } }
}

impl<const N:usize> Node<N> for Powf {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        match inputs.len() {
            1 => {
                for i in 0..N {
                    output[0][i] = inputs[0].buffers()[0][i].powf(self.val);
                }
            },
            2 => {
                let mut main = &inputs[1];
                let mut sidechain = &inputs[0];
                
                if self.main_input == inputs[0].node_id {
                    main = &inputs[0];
                    sidechain = &inputs[1];
                }
                
                if self.sidechain_input == inputs[1].node_id {
                    main = &inputs[0];
                    sidechain = &inputs[1];
                }
                
                for i in 0..N {
                    output[0][i] = main.buffers()[0][i].powf(
                        sidechain.buffers()[0][i]
                    );
                }
            },
            _ => {}
        }
    }
    fn send_msg(&mut self, info: Message) {
        match info {
            Message::MainInput(v) => {
                self.main_input = v.index();
                
            },
            Message::SidechainInput(v) => {
                self.sidechain_input = v.index();
            },
            _ => {}
        }
    }
}

fn main() {
    let mut context = AudioContextBuilder::<32>::new().channels(1).build();

    let node_index_a = context.add_mono_node( ConstSig::new(2.0) );

    // node b is a powf node, see its def above
    // when it has two input,
    // it use the main input as the base, to the power of the sidechain input
    let node_index_b = context.add_mono_node( Powf::new(1.0) );
    let node_index_c = context.add_mono_node( ConstSig::new(3.0) );

    // node a goes to b first, so node a is the main input of node b
    let _ = context.connect(node_index_a, node_index_b);
    // node c goes to b second, so node c is the sidechain input
    let _ = context.connect(node_index_c, node_index_b);
    let _ = context.connect(node_index_b, context.destination);

    // get lots of 8.0, because main input is 2, sidechain is 3
    // 2 to the power of 3, that's 8
    println!("result {:?}", context.next_block());

    // we can set node c as the main input of b
    context.send_msg(node_index_b, Message::MainInput(node_index_c));

    // then we get a lot of 9.0
    // as the main input is now 3.0; sidechain is 2.0
    // 3.0^2.0 = 9.0
    println!("result {:?}", context.next_block());
}