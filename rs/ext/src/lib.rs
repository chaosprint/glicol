#![allow(warnings)]
use glicol_macro::*;
use glicol_synth::{SimpleGraph, GlicolNodeData, GlicolError};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
use pest::iterators::Pairs;
use std::{collections::HashMap};


// def_node!("bd", [Modulable],
//     to_sink: ~out1 // >> add ~out2
//     ~envb: ~triggerb >> envperc 0.01 PARA_0;
//     ~env_pitch: ~triggerb >> envperc 0.01 0.1;
//     ~pitch: ~env_pitch >> mul 50 >> add 60;
//     ~triggerb: [[seq 60]];
//     ~out1: [[sin ~pitch >> mul ~envb >> mul 0.8]]
// )


def_node!("bd", [Fixed(0.2)], {
    let decay = args[0] * 1.0; // rust code
}, {
    SINK: _out1 // >> add ~out2

    _envb: _triggerb >> envperc 0.01 #decay;

    _env_pitch: _triggerb >> envperc 0.01 0.1;

    _pitch: _env_pitch >> mul 50 >> add 60;

    _triggerb: SOURCE;

    _out1: sin _pitch >> mul _envb >> mul 0.8;
});

// def_node!("mul", [Modulable(100.0)], {
//         let freq = args[0];
//     }
//     CHAIN_NAME: SOURCE >> mul ~modulation >> SINK;
//     ~modulation: const_sig PARA1
// );

register_extensions! {
    Plate: 1,
    Kick: 2,
    Bd: 1,
    Hh: 1,
    Sn: 1,
    Ks: 3,
    Sawsynth: 2,
    Squsynth: 2,
    Trisynth: 2,
}

// remember to regitster on glicol_parser too!
// write the documentation (node description, parameter names and number, etc.) on glicol-js/glicol-docs.json