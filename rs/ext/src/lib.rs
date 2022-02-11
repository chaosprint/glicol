#![allow(warnings)]
use glicol_macro::*;
use glicol_synth::{SimpleGraph, GlicolNodeData, GlicolError, Para};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
use pest::iterators::Pairs;
use std::{collections::HashMap};

def_node!({
    "sawsynth": {
        args: [Para::Number(0.001), Para::Number(0.1)], 
        paras: {
            let attack = &args[0];
            let decay = &args[1];
        },
        graph: {
            output: saw ~pitch >> mul ~env;
            ~trigger: ~input;
            ~pitch: ~trigger >> mul 261.626;
            ~env: ~trigger >> envperc #attack #decay;
        }
    },
    "bd": {
        args: [Para::Number(0.3)], 
        paras: {
            let decay = &args[0];
        },
        graph: {
            output: sin ~pitch >> mul ~envb >> mul 0.8;
            ~envb: ~triggerb >> envperc 0.01 #decay;
            ~env_pitch: ~triggerb >> envperc 0.01 0.1;
            ~pitch: ~env_pitch >> mul 50 >> add 60;
            ~triggerb: ~input;
        }
    },
    "plate": {
        args: [Para::Number(0.1)],
        paras: {
            let mix = &args[0];
            let mixdiff = 1. - mix.parse::<f32>().unwrap();
        },
        graph: {
            ~dry: ~input;
            ~wet: ~dry >> onepole 0.7
            >> delay 0.05 >> apfgain 0.004771 0.75 >> apfgain 0.003595 0.75
            >> apfgain 0.01272 0.625 >> apfgain 0.009307 0.625
            >> add ~back
            >> apfgain ~modu 0.7;
            ~modu: sin 0.1 >> mul 0.0055 >> add 0.0295;
            ~aa: ~wet >> delayn 394.0;
            ~ab: ~aa >> delayn 2800.0;
            ~ac: ~ab >> delayn 1204.0;
            ~ba: ~ac >> delayn 2000.0 >> onepole 0.1
            >> apfgain 0.007596 0.5;
            ~bb: ~ba >> apfgain 0.03578 0.5;
            ~bc: ~bb >> apfgain ~modu 0.5;
            ~ca: ~bc >> delayn 179.0;
            ~cb: ~ca >> delayn 2679.0;
            ~cc: ~cb >> delayn 3500.0 >> mul 0.3;
            ~da: ~cc >> apfgain 0.03 0.7 >> delayn 522.0;
            ~db: ~da >> delayn 2400.0;
            ~dc: ~db >> delayn 2400.0;
            ~ea: ~dc >> onepole 0.1 >> apfgain 0.0062 0.7;
            ~eb: ~ea >> apfgain 0.03492 0.7;
            ~fa: ~eb >> apfgain 0.0204 0.7 >> delayn 1578.0;
            ~fb: ~fa >> delayn 2378.0;
            ~back: ~fb >> delayn 2500.0 >> mul 0.3;
            
            ~subtract_left: ~bb >> add ~db >> add ~ea >> add ~fa >> mul -1.0;
            
            ~left: ~aa >> add ~ab >> add ~cb >> add ~subtract_left
            >> mul #mix >> add ~drym;
            
            ~sub_right: ~eb >> add ~ab >> add ~ba >> add ~ca >> mul -1.0;
            
            ~right: ~da >> add ~db >> add ~fb >> add ~sub_right
            >> mul #mix >> add ~drym;
            
            ~drym: ~dry >> mul #mixdiff;
            
            output: balance ~left 0.5 ~right 0.5;
        }
    }
});



// let args = get_args(paras, mod_info);
// let xx = args[0]
// ..
// let appendix_body = format!();


// def_node add nodes info, struct to a hashmap,
// this hashmap provides tools to output the node code




// , {
//     synth: saw ~pitch >> mul ~env;
//     ~trigger: ~input;
//     ~pitch: ~trigger >> mul 261.626;
//     ~env: ~trigger >> envperc #attack #decay;
// }
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