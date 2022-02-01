#![allow(warnings)]
use glicol_macro::*;
use glicol_synth::{SimpleGraph, GlicolNodeData};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
use pest::iterators::Pairs;
use std::{collections::HashMap};


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