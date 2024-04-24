use glicol_synth::GlicolPara;
use pest::iterators::Pair;

use crate::Rule;

pub fn two_numbers(
    node: Pair<'_, Rule>,
    chain_paras: &mut Vec<Vec<GlicolPara>>
) {
    // println!("node {:?}", node.as_str());
    let mut iter = node.into_inner();
    let p1 = iter.next().unwrap();
    let p2 = iter.next().unwrap();
    chain_paras.push(vec![
        GlicolPara::Number(p1.as_str().parse::<f32>().unwrap()),
        GlicolPara::Number(p2.as_str().parse::<f32>().unwrap())
    ]);
}

pub fn one_para_number_or_ref(
    node: Pair<'_, Rule>,
    chain_paras: &mut Vec<Vec<GlicolPara>>
) {
    // println!("node {:?}", node.as_str()); //"sin 440"
    let paras = node.into_inner().next().unwrap();
    // println!("paras {:?}", paras.as_str());//"440"
    match paras.as_rule() {
        Rule::number => {
            chain_paras.push(vec![GlicolPara::Number(paras.as_str().parse::<f32>().unwrap())]);
        },
        Rule::reference => {
            chain_paras.push(vec![GlicolPara::Reference(paras.as_str().to_owned())]);
        },
        _ => {}
    }
}
