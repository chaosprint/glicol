use hashbrown::HashMap;
use nodes::{Component, Node as _, Points};
use pest::error::Error;
use pest::Parser;
use pest_derive::*;
use util::{EndSpan, ToPestErrWithPositives};

pub mod nodes;
mod util;
pub use util::ToInnerOwned;

#[derive(Parser)]
#[grammar = "glicol.pest"]
pub struct GlicolParser;

pub fn get_ast(code: &str) -> Result<nodes::Ast<'_>, Box<Error<Rule>>> {
    let mut block = GlicolParser::parse(Rule::block, code)?;

    // this can be a comment though, but we call it a line
    let lines = block.next().ok_or_else(|| {
        pest::Span::new(code, 0, 0)
            // We know this will be a valid span b/c 0 (end) >= 0 (start)
            .unwrap()
            .to_err_with_positives([Rule::line])
    })?;

    //for line in lines.into_inner() {
    let nodes = lines.into_inner()
        .filter(|line| line.as_rule() == Rule::line)
        .map(|line| {
            let line_end = line.as_end_span();
            let mut comp_iter = line.into_inner();

            let ref_pair = comp_iter.next()
                // make sure it's a reference
                .and_then(|r| (r.as_rule() == Rule::reference).then_some(r))
                // if it's not, then report an error
                .ok_or_else(|| line_end.to_err_with_positives([Rule::reference]))?;

            let name = ref_pair.as_str();

            let chain  = comp_iter.next()
                // make sure it's a chain
                .and_then(|r| (r.as_rule() == Rule::chain).then_some(r))
                // if it's not, then report an error
                .ok_or_else(|| line_end.to_err_with_positives([Rule::chain]))?;

            let components = chain.into_inner().map(|node_pair| {
                let node_end = node_pair.as_end_span();
                let node = node_pair.into_inner()
                    .next()
                    .ok_or_else(|| node_end.to_err_with_positives([Rule::node]))?;

                let component = match_or_return_err!(node,
                    Rule::points => { Component::Points(Points::parse(node)?) },
                    Rule::delayn => { Component::Delayn(nodes::Delayn::parse(node)?) },
                    Rule::delayms => { Component::Delayms(nodes::Delayms::parse(node)?) },
                    Rule::imp => { Component::Imp(nodes::Imp::parse(node)?) },
                    Rule::tri => { Component::Tri(nodes::Tri::parse(node)?) },
                    Rule::squ => { Component::Squ(nodes::Squ::parse(node)?) },
                    Rule::saw => { Component::Saw(nodes::Saw::parse(node)?) },
                    Rule::onepole => { Component::Onepole(nodes::Onepole::parse(node)?) },
                    Rule::sin => { Component::Sin(nodes::Sin::parse(node)?) },
                    Rule::mul => { Component::Mul(nodes::Mul::parse(node)?) },
                    Rule::add => { Component::Add(nodes::Add::parse(node)?) },
                    Rule::pan => { Component::Pan(nodes::Pan::parse(node)?) },
                    Rule::seq => { Component::Seq(nodes::Seq::parse(node)?) },
                    Rule::choose => { Component::Choose(nodes::Choose::parse(node)?) },
                    Rule::mix => { Component::Mix(nodes::Mix::parse(node)?) },
                    Rule::sp => { Component::Sp(nodes::Sp::parse(node)?) },
                    Rule::speed => { Component::Speed(nodes::Speed::parse(node)?) },
                    Rule::constsig => { Component::ConstSig(nodes::ConstSig::parse(node)?) },
                    Rule::adc => { Component::Adc(nodes::Adc::parse(node)?) },
                    Rule::bd => { Component::Bd(nodes::Bd::parse(node)?) },
                    Rule::sn => { Component::Sn(nodes::Sn::parse(node)?) },
                    Rule::hh => { Component::Hh(nodes::Hh::parse(node)?) },
                    Rule::sawsynth => { Component::SawSynth(nodes::SawSynth::parse(node)?) },
                    Rule::squsynth => { Component::SquSynth(nodes::SquSynth::parse(node)?) },
                    Rule::trisynth => { Component::TriSynth(nodes::TriSynth::parse(node)?) },
                    Rule::lpf => { Component::Lpf(nodes::Lpf::parse(node)?) },
                    Rule::psampler => { Component::PSampler(nodes::PSampler::parse(node)?) },
                    Rule::balance => { Component::Balance(nodes::Balance::parse(node)?) },
                    Rule::rhpf => { Component::Rhpf(nodes::Rhpf::parse(node)?) },
                    Rule::apfmsgain => { Component::ApfmsGain(nodes::ApfmsGain::parse(node)?) },
                    Rule::reverb => { Component::Reverb(nodes::Reverb::parse(node)?) },
                    Rule::envperc => { Component::EnvPerc(nodes::EnvPerc::parse(node)?) },
                    Rule::adsr => { Component::Adsr(nodes::Adsr::parse(node)?) },
                    Rule::plate => { Component::Plate(nodes::Plate::parse(node)?) },
                    Rule::get => { Component::Get(nodes::Get::parse(node)?) },
                    Rule::noise => { Component::Noise(nodes::Noise::parse(node)?) },
                    Rule::meta => { Component::Meta(nodes::Meta::parse(node)?) },
                    Rule::expr => { Component::Expr(nodes::Expr::parse(node)?) },
                    Rule::eval => { Component::Eval(nodes::Eval::parse(node)?) },
                    Rule::arrange => { Component::Arrange(nodes::Arrange::parse(node)?) },
                    Rule::msgsynth => { Component::MsgSynth(nodes::MsgSynth::parse(node)?) },
                    Rule::pattern_synth => { Component::PatternSynth(nodes::PatternSynth::parse(node)?) },
                );

                Ok(component)
            }).collect::<Result<Vec<_>, _>>()?;

            Result::<_, Box<Error<Rule>>>::Ok((
                name,
                components
            ))
        }).collect::<Result<HashMap<_, _>, _>>()?;

    Ok(nodes::Ast { nodes })
}
