#[allow(dead_code)]
use {
    nom::IResult,
    nom::branch::alt,
    nom::branch::permutation,
    nom::character::complete::digit1,
    nom::character::complete::alpha1,
    nom::character::complete::space0,
    nom::character::complete::space1,
    nom::bytes::complete::tag,
    nom::number::complete::float,
    nom::multi::separated_list1,
    nom::sequence::tuple,
    nom::combinator::map,
    nom::Finish,
    nom::number::complete::recognize_float,
    nom::character::complete::alphanumeric1,
    nom::multi::many0_count,
    nom::sequence::pair,
    nom::combinator::recognize,
    nom::sequence::separated_pair,
};

// use std::collections::HashMap;


// #[derive(Debug, Copy, Clone)]
// pub enum GlicolNodeInfo<'a> {
//     ConstSig(&'a str),
//     SinOsc(&'a str),
//     Mul(&'a str),
//     Add(&'a str),
//     Phasor(&'a str),
// }

pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(
      pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_"))))
      )
    )(input)
}

// the return should be sth like: [("sin", "440.0"), ("mul", "0.1")]
pub fn nodes(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    map(
        separated_list1(
            tuple((space0, tag(">>"), space0)), // seperator
            tuple( // node such as "sin 440.0"
                (
                    alt(
                        (
                            tag("sin"),
                            tag("const_sig"),
                            tag("squ"),
                            tag("tri"),
                            tag("mul"),
                            tag("add"),
                        )
                    ),
                    space1,
                    alt((recognize_float, identifier))
                )
            )),
        |nodelist: Vec<(&str, &str, &str)>| {
            nodelist.into_iter().map(|c:(&str, &str, &str)| {
                (c.0, c.2)
                // match c.0 {
                //     "sin" => GlicolNodeInfo::SinOsc(c.2),
                //     "mul" => GlicolNodeInfo::Mul(c.2),
                //     "add" => GlicolNodeInfo::Add(c.2),
                //     "const_sig" => GlicolNodeInfo::ConstSig(c.2),
                //     // "saw" => GlicolNode::SawOsc(c.2),
                //     // "squ" => GlicolNode::SquOsc(c.2),
                //     // "tri" => GlicolNode::TriOsc(c.2),
                //     // "mul" => GlicolNode::Mul(c.2),
                //     // "add" => GlicolNode::Add(c.2),
                //     _ => unimplemented!()
                // }
            }).collect::<Vec<_>>()
        }
    )(input)
}

pub fn single_chain(input: &str) -> IResult<&str, (&str, Vec<(&str, &str)>)> {
    map(tuple((identifier, recognize(separated_pair(space0, tag(":"), space0)), nodes)),
    |chain_info: (&str, &str, Vec<(&str, &str)>) | {
        (chain_info.0, chain_info.2)
    })(input)
}