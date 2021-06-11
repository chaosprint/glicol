#[macro_export]
macro_rules! mono_node {
    ($body:expr) => {
        NodeData::new1( BoxedNodeSend::new(($body)))
    };
}

#[macro_export]
macro_rules! stereo_node {
    ($body:expr) => {
        NodeData::new2( BoxedNodeSend::new(($body)))
    };
}

#[macro_export]
macro_rules! ndef {
    ($struct_name: ident, $channel_num: ident, {$code_str: expr}) => {
        pub struct $struct_name {
            engine: Engine
        }
        
        impl $struct_name {
            pub fn new(paras: &mut Pairs<Rule>) -> Result<
            (NodeData<BoxedNodeSend<128>, 128>, Vec<String>), EngineError> {
                let mut engine = Engine::new();
                engine.set_code(&format!($code_str, a=paras.as_str()));
                engine.make_graph()?;
                Ok((NodeData::$channel_num(BoxedNodeSend::new( Self {
                    engine
                })), vec![]))
            }
        }
        
        impl Node<128> for $struct_name {
            fn process(&mut self, inputs: &[Input<128>], output: &mut [Buffer<128>]) {
                // self.engine.input(inputs); // mono or stereo?
                let mut input = inputs[0].buffers()[0].clone();
                let buf = self.engine.gen_next_buf_128(&mut input).unwrap();
                match output.len() {
                    1 => {
                        for i in 0..128 {
                            output[0][i] = buf.0[i];
                        }
                    },
                    2 => {
                        for i in 0..128 {
                            output[0][i] = buf.0[i];
                            output[1][i] = buf.0[i+128];
                        }
                    },
                    _ => {}
                }
            }
        }
    };
}

#[macro_export]
macro_rules! imp {
    ({$($para: ident: $data:expr),*}) => {
         (
            Impulse::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! noise {
    () => { // controlled by modulator, no need for value
        Noise::new(42)
    };

    ($data: expr) => {
        Noise::new($data)
    };
}

#[macro_export]
macro_rules! speed {
    ($data: expr) => {
        Speed::new($data)
    };
}

#[macro_export]
macro_rules! mul {
    () => { // controlled by modulator, no need for value
        Mul::new(0.0)
    };

    ($data: expr) => {
        Mul::new($data)
    };
}


#[macro_export]
macro_rules! sin_osc {
    ({$($para: ident: $data:expr),*}) => {
         (
            SinOsc::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! tri_osc {
    ({$($para: ident: $data:expr),*}) => {
         (
            TriOsc::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! squ_osc {
    ({$($para: ident: $data:expr),*}) => {
         (
            SquOsc::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! saw_osc {
    ({$($para: ident: $data:expr),*}) => {
         (
            SawOsc::new()$(.$para($data))*.build()
        )
    }
}
