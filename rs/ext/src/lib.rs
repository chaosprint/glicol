#![allow(warnings)]
use glicol_macro::*;
use glicol_synth::{SimpleGraph, GlicolNodeData, GlicolError, Para};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
use pest::iterators::Pairs;
use std::{collections::HashMap};

def_node!({
    "sawsynth": {
        args: [Fixed(0.001), Fixed(0.1)], 
        paras: {
            let attack = args[0];
            let decay = args[1];
        },
        graph: {
            output: saw ~pitch >> mul ~env;
            ~trigger: ~input;
            ~pitch: ~trigger >> mul 261.626;
            ~env: ~trigger >> envperc #attack #decay;
        }
    },
    "bd": {
        args: [Fixed(0.001), Fixed(0.1)], 
        paras: {
            let attack = args[0];
            let decay = args[1];
        },
        graph: {
            output: saw ~pitch >> mul ~env;
            ~trigger: ~input;
            ~pitch: ~trigger >> mul 261.626;
            ~env: ~trigger >> envperc #attack #decay;
        }
    }
});

// def_node add nodes info, struct to a hashmap,
// this hashmap provides tools to output the node code

pub fn preprocess2(mut code: &mut String) -> Result<String, GlicolError> {
    let mut target_code = code.clone();
    let lines = match GlicolParser::parse(Rule::block, &mut code) {
        Ok(mut res) => {
            if res.as_str() < &mut target_code {
                unimplemented!();
            }
            res.next().unwrap()
        },
        Err(e) => { unimplemented!()}
    };
    let mut processed_code = "".to_owned();
    let mut appendix_full = "".to_owned();
    let mut current_ref_name = "".to_owned();

    for line in lines.into_inner() {
        let inner_rules = line.into_inner();
        for element in inner_rules {
            match element.as_rule() {
                Rule::reference => {
                    current_ref_name = element.as_str().to_owned();
                    processed_code.push_str("\n");
                    processed_code.push_str(&current_ref_name);
                    processed_code.push_str(": ");
                    // println!("current_ref_name {:?}", current_ref_name);
                },
                Rule::chain => {
                    let mut node_str_list = vec![];
                    for node in element.into_inner() {
                        let mut name_and_paras = node.into_inner();
                        let name_and_paras_str: String = name_and_paras.as_str().to_string();
                        let node_name = name_and_paras.next().unwrap();
                        let name = node_name.as_str().clone();
                        let mut paras = name_and_paras.clone(); // the name is ripped aboves
                        if vec!["sawsynth"].contains(&name) {
                            
                            let appendix_body = match name {
                                "sawsynth" => "output: saw ~pitch >> mul ~env;~trigger: ~input;~pitch: ~trigger >> mul 261.626;~env: ~trigger >> envperc 0.001 0.1;",
                                _ => {unimplemented!()}
                            };

                            let mut appendix = appendix_body.replace("~input", &node_str_list.join(" >> "));
                            let mut ref_receiver = "~".to_owned();
                            ref_receiver.push_str(&current_ref_name.replace("~", ""));
                            ref_receiver.push_str("_");
                            ref_receiver.push_str(name);
                            let mut appendix = appendix.replace("output", &ref_receiver);
                            appendix_full.push_str(&appendix);
                            appendix_full.push_str("\n\n");
                            node_str_list = vec![ref_receiver];
                        } else {
                            let mut s = name.to_owned();
                            s.push_str(" ");
                            s.push_str(paras.as_str());
                            node_str_list.push(s);
                        };
                    }
                    processed_code.push_str(&node_str_list.join(" >> "));
                    processed_code.push_str("\n\n");
                    processed_code.push_str(&appendix_full);
                },
                _ => {}
            }
        };
        
    };
    Ok(processed_code)
}


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