// #[macro_export]
// macro_rules! mono_node {
//     ($body:expr) => {
//         NodeData::new1( BoxedNodeSend::new(($body)))
//     };
// }

#[macro_export]
macro_rules! mono_node {
    ($size:expr, $body:expr) => {
        NodeData::new1( BoxedNodeSend::<$size>::new(($body)))
    };
}

#[macro_export]
macro_rules! stereo_node {
    ($size:expr, $body:expr) => {
        NodeData::new2( BoxedNodeSend::<$size>::new(($body)))
    };
}

#[macro_export]
macro_rules! sin_osc {
    ($size:expr => {$($para: ident: $data:expr),*  }) => {
         (
            SinOsc::<$size>::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! noise {

    // ($size: expr) => {
    //     Noise::<$size>::new(42)
    // };
    ($size: expr => $data: expr) => {
        Noise::<$size>::new($data)
    };
}


#[macro_export]
macro_rules! mul {

    ($size: expr) => {
        Mul::<$size>::new(0.0)
    };
    ($size: expr => $data: expr) => {
        Mul::<$size>::new($data)
    };
}


#[macro_export]
macro_rules! imp {
    ($size:expr => {$($para: ident: $data:expr),*}) => {
         (
            Impulse::<$size>::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! speed {
    ($size:expr => $data: expr) => {
        Speed::<$size>::new($data)
    };
}

#[macro_export]
macro_rules! tri_osc {
    ($size:expr => {$($para: ident: $data:expr),*}) => {
         (
            TriOsc::<$size>::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! squ_osc {
    ($size:expr =>{$($para: ident: $data:expr),*}) => {
         (
            SquOsc::<$size>::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! saw_osc {
    ($size:expr => {$($para: ident: $data:expr),*}) => {
         (
            SawOsc::<$size>::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! const_sig {
    ($size:expr => $data: expr) => {
        ConstSig::<$size>::new($data)
    };
}


#[macro_export]
macro_rules! seq {
    ($size:expr => {$($para: ident: $data:expr),*}) => {
         (
            Sequencer::<$size>::new()$(.$para($data))*.build()
        )
    }
}


#[macro_export]
macro_rules! sampler {
    ($size:expr => $data: expr) => {
        Sampler::<$size>::new($data)
    };
}


#[macro_export]
macro_rules! choose {
    ($size:expr => $data: expr) => {
        Choose::<$size>::new($data);
    };
}


#[macro_export]
macro_rules! apfdecay {
    ($size:expr => {$($para: ident: $data:expr),*}) => {
         (
            AllpassDecay::<$size>::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! apfgain {
    ($size:expr => {$($para: ident: $data:expr),*}) => {
         (
            AllpassGain::<$size>::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! comb {
    ($size:expr => {$($para: ident: $data:expr),*}) => {
            (
            Comb::<$size>::new()$(.$para($data))*.build()
        )
    }
}


#[macro_export]
macro_rules! onepole {
    ($size:expr => $data: expr) => {
        OnePole::<$size>::new($data);
    };
}


#[macro_export]
macro_rules! rhpf {
    ($size:expr => {$($para: ident: $data:expr),*}) => {
         (
            ResonantHighPassFilter::<$size>::new()$(.$para($data))*.build()
        )
    }
}


#[macro_export]
macro_rules! rlpf {
    ($size:expr => {$($para: ident: $data:expr),*}) => {
         (
            ResonantLowPassFilter::<$size>::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! envperc {
    ($size:expr => {$($para: ident: $data:expr),*}) => {
         (
            EnvPerc::<$size>::new()$(.$para($data))*.build()
        )
    }
}


#[macro_export]
macro_rules! balance {
    // () => { // controlled by modulator, no need for value
    //     Balance::new(0.5)
    // };

    ($size:expr => $data: expr) => {
        Balance::<$size>::new($data)
    };
}

#[macro_export]
macro_rules! delay {
    ($size:expr => {$($para: ident: $data:expr),*}) => {
         (
            Delay::<$size>::new()$(.$para($data))*.build()
        )
    }
}

#[macro_export]
macro_rules! delayn {
    ($size:expr => $data: expr) => {
        DelayN::<$size>::new($data);
    };
}


#[macro_export]
macro_rules! pan {
    // () => { // controlled by modulator, no need for value
    //     Pan::new(0.5)
    // };

    ($size:expr => $data: expr) => {
        Pan::<$size>::new($data)
    };
}

#[macro_export]
macro_rules! add {
    // () => {
    //     Add::new(0.0)
    // };

    ($size:expr =>$data: expr) => {
        Add::<$size>::new($data)
    };
}
