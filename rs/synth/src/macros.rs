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
