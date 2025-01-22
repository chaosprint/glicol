use hashbrown::HashMap;
use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Span,
};

use crate::{
    match_or_return_err,
    util::{EndSpan, GetNextParsed, ToPestErrWithPositives, TryToParse},
    Rule,
};

#[cfg(test)]
trace::init_depth_var!();

#[derive(yoke::Yokeable, Debug, PartialEq)]
pub struct Ast<'ast> {
    pub nodes: HashMap<&'ast str, Vec<Component<'ast>>>,
}

pub trait Node<'ast>
where
    Self: Sized,
{
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>>;
    fn parse(pair: Pair<'ast, Rule>) -> Result<Self, Box<Error<Rule>>> {
        let span = pair.as_span();
        Self::parse_from_iter(&mut pair.into_inner(), span)
    }
}

pub trait SingleNodeItem<'ast> {
    type Item: Node<'ast>;
    fn from_item(item: Self::Item) -> Self;
}

impl<'ast, T> Node<'ast> for T
where
    T: SingleNodeItem<'ast>,
{
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        Ok(Self::from_item(T::Item::parse_from_iter(pairs, span)?))
    }
}

#[derive(PartialEq, Debug)]
pub enum Component<'ast> {
    Points(Points),
    Delayn(Delayn<'ast>),
    Delayms(Delayms<'ast>),
    Imp(Imp<'ast>),
    Tri(Tri<'ast>),
    Squ(Squ<'ast>),
    Saw(Saw<'ast>),
    Onepole(Onepole<'ast>),
    Sin(Sin<'ast>),
    Mul(Mul<'ast>),
    Add(Add<'ast>),
    Pan(Pan<'ast>),
    Seq(Seq<'ast>),
    Choose(Choose),
    Arrange(Arrange<'ast>),
    Mix(Mix<'ast>),
    Sp(Sp<'ast>),
    Speed(Speed),
    ConstSig(ConstSig),
    Adc(Adc),
    Bd(Bd<'ast>),
    Sn(Sn<'ast>),
    Hh(Hh<'ast>),
    SawSynth(SawSynth),
    SquSynth(SquSynth),
    TriSynth(TriSynth),
    MsgSynth(MsgSynth<'ast>),
    PatternSynth(PatternSynth<'ast>),
    Lpf(Lpf<'ast>),
    PSampler(PSampler<'ast>),
    Balance(Balance<'ast>),
    Rhpf(Rhpf<'ast>),
    ApfmsGain(ApfmsGain<'ast>),
    Reverb(Reverb),
    Plate(Plate),
    EnvPerc(EnvPerc),
    Adsr(Adsr),
    Get(Get<'ast>),
    Noise(Noise),
    Meta(Meta<'ast>),
    Expr(Expr<'ast>),
    Eval(Eval<'ast>),
}

impl<'ast> Component<'ast> {
    pub fn all_references<'a>(&'a self) -> Vec<&'ast str> {
        match self {
            Self::Delayn(Delayn {
                param: UsizeOrRef::Ref(r),
            })
            | Self::Delayms(Delayms {
                param: NumberOrRef::Ref(r),
            })
            | Self::Imp(Imp {
                param: NumberOrRef::Ref(r),
            })
            | Self::Tri(Tri {
                param: NumberOrRef::Ref(r),
            })
            | Self::Squ(Squ {
                param: NumberOrRef::Ref(r),
            })
            | Self::Saw(Saw {
                param: NumberOrRef::Ref(r),
            })
            | Self::Onepole(Onepole {
                param: NumberOrRef::Ref(r),
            })
            | Self::Sin(Sin {
                param: NumberOrRef::Ref(r),
            })
            | Self::Mul(Mul {
                param: NumberOrRef::Ref(r),
            })
            | Self::Add(Add {
                param: NumberOrRef::Ref(r),
            })
            | Self::Pan(Pan {
                param: NumberOrRef::Ref(r),
            })
            | Self::Bd(Bd {
                param: NumberOrRef::Ref(r),
            })
            | Self::Sn(Sn {
                param: NumberOrRef::Ref(r),
            })
            | Self::Hh(Hh {
                param: NumberOrRef::Ref(r),
            })
            | Self::Lpf(Lpf {
                signal: Signal::Reference(r),
                qvalue: _,
            })
            | Self::Rhpf(Rhpf {
                cutoff: NumberOrRef::Ref(r),
                qvalue: _,
            })
            | Self::ApfmsGain(ApfmsGain {
                delay: NumberOrRef::Ref(r),
                gain: _,
            })
            | Self::Get(Get { reference: r }) => vec![r],

            Self::Seq(Seq { events }) => events
                .iter()
                .flat_map(|(_, e)| match e {
                    UsizeOrRef::Usize(_) => None,
                    UsizeOrRef::Ref(r) => Some(*r),
                })
                .collect(),
            Self::Arrange(Arrange { events }) => events
                .iter()
                .flat_map(|e| match e {
                    NumberOrRef::Number(_) => None,
                    NumberOrRef::Ref(r) => Some(*r),
                })
                .collect(),
            Self::Mix(Mix { nodes }) => nodes.clone(),
            Self::Balance(Balance { left, right }) => vec![left, right],

            // mmm I don't like using wildcard matches but it's definitely the most convenient in
            // this situation so here we are
            _ => vec![],
        }
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum Duration {
    Bar(f32),
    Seconds(f32),
    Milliseconds(f32),
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub struct TimeList {
    pub bar: f32,
    pub time: Option<Duration>,
}

#[derive(PartialEq, Debug)]
pub struct Points {
    pub points: Vec<(TimeList, f32)>,
    pub span: f32,
    pub is_looping: bool,
}

impl Node<'_> for Points {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Points]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let mut node_span = -1.0;
        let mut is_looping = false;
        let points = pairs.next().ok_or_else(|| {
            span.as_end_span()
                .to_err_with_positives([Rule::points_inner])
        })?;

        if let Some(math_or_loop) = pairs.next() {
            match_or_return_err!(math_or_loop,
                Rule::math_expression => {
                    let mut one = "1".to_owned();
                    let mut ns = fasteval::EmptyNamespace;
                    one.push_str(math_or_loop.as_str());
                    node_span = fasteval::ez_eval(&one, &mut ns).unwrap() as f32;
                    if pairs.next().is_some() {
                        is_looping = true;
                    };
                },
                Rule::is_looping => {
                    node_span = 1.0;
                    is_looping = true;
                },
            )
        }

        let points = points
            .into_inner()
            .map(|point| {
                let end_span = point.as_end_span();
                let mut point_inner = point.into_inner();

                let time = point_inner
                    .next()
                    .ok_or_else(|| end_span.to_err_with_positives([Rule::time]))?;

                let time_end = time.as_end_span();
                let mut time_inner = time.into_inner();
                let bar = time_inner
                    .next()
                    .ok_or_else(|| end_span.to_err_with_positives([Rule::number, Rule::bar]))?;

                let bar = match_or_return_err!(bar,
                    Rule::number => {
                        bar.try_to_parse()?
                    },
                    Rule::bar => {
                        let bar_end = bar.as_end_span();
                        let mut nums = bar.into_inner();

                        let top = nums.next_parsed::<f32>(bar_end)?;
                        let bottom = nums.next_parsed::<f32>(bar_end)?;

                        top / bottom
                    },
                );

                let time_res = time_inner.next().map(|sign_rule| {
                    let sign = if sign_rule.as_str() == "-" { -1.0 } else { 1.0 };

                    let s = time_inner
                        .next()
                        .ok_or_else(|| time_end.to_err_with_positives([Rule::second, Rule::ms]))?;
                    let s_end = s.as_end_span();

                    // clippy has a false positive on these return statements - they are needed.
                    match_or_return_err!(s,
                        Rule::second => {
                            #[allow(clippy::needless_return)]
                            return Ok(Duration::Seconds(
                                sign * s.into_inner().next_parsed::<f32>(s_end)?
                            ))
                        },
                        Rule::ms => {
                            #[allow(clippy::needless_return)]
                            return Ok(Duration::Milliseconds(
                                sign * s.into_inner().next_parsed::<f32>(s_end)?
                            ))
                        },
                    );
                });

                let time = match time_res {
                    Some(Err(e)) => return Err(e),
                    Some(Ok(t)) => Some(t),
                    None => None,
                };

                let value = point_inner.next_parsed(end_span)?;

                Ok((TimeList { bar, time }, value))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            points,
            span: node_span,
            is_looping,
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NumberOrRef<S>
where
    S: AsRef<str>,
{
    Number(f32),
    Ref(S),
}

impl<'ast> Node<'ast> for NumberOrRef<&'ast str> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ NumberOrRef]"))]
    fn parse(pair: Pair<'ast, Rule>) -> Result<Self, Box<Error<Rule>>> {
        match_or_return_err!(pair,
            Rule::number => {
                pair.try_to_parse().map(Self::Number)
            },
            Rule::reference => {
                Ok(Self::Ref(pair.as_str()))
            },
        )
    }

    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        pairs
            .next()
            .ok_or_else(|| {
                span.as_end_span()
                    .to_err_with_positives([Rule::reference, Rule::number])
            })
            .and_then(Self::parse)
    }
}

macro_rules! impl_single_item_classes{
    ($(($($class:ident,)*) => $param:ident: $item:ty,)*) => {
        $($(
            #[derive(PartialEq, Debug)]
            pub struct $class<'ast> {
                pub $param: $item
            }

            impl<'ast> SingleNodeItem<'ast> for $class<'ast> {
                type Item = $item;
                fn from_item($param: Self::Item) -> Self {
                    Self { $param }
                }
            }
        )*)*
    }
}

impl_single_item_classes!(
    (
        Delayn,
    ) => param: UsizeOrRef<&'ast str>,
    (
        Delayms,
        Imp,
        Tri,
        Squ,
        Saw,
        Onepole,
        Sin,
        Mul,
        Add,
        Pan,
        Bd,
        Sn,
        Hh,
    ) => param: NumberOrRef<&'ast str>,
    (
        Meta,
        Expr,
        Eval,
    ) => code: CodeBlock<'ast>,
);

#[derive(PartialEq, Debug)]
pub struct Speed {
    pub speed: f32,
}

impl Node<'_> for Speed {
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        pairs.next_parsed(span).map(|speed| Self { speed })
    }
}

#[derive(PartialEq, Debug)]
pub struct Get<'ast> {
    pub reference: &'ast str,
}

impl<'ast> Node<'ast> for Get<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Get]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        pairs
            .next()
            .ok_or_else(|| span.as_end_span().to_err_with_positives([Rule::reference]))
            .map(|p| Self {
                reference: p.as_str(),
            })
    }
}

#[derive(PartialEq, Debug)]
pub struct Noise {
    pub seed: usize,
}

impl Node<'_> for Noise {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Noise]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        pairs.next_parsed(span).map(|seed| Self { seed })
    }
}

#[derive(PartialEq, Debug, PartialOrd, Clone)]
pub enum UsizeOrRef<S>
where
    S: AsRef<str>,
{
    Usize(usize),
    Ref(S),
}

impl<'ast> Node<'ast> for UsizeOrRef<&'ast str> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ UsizeOrRef]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let next = pairs.next().ok_or_else(|| {
            span.as_end_span()
                .to_err_with_positives([Rule::integer, Rule::reference])
        })?;

        match_or_return_err!(next,
            Rule::number => {
                next.try_to_parse().map(Self::Usize)
            },
            Rule::reference => {
                Ok(Self::Ref(next.as_str()))
            },
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct Adc {
    pub port: u32,
}

impl Node<'_> for Adc {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Adc]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        pairs.next_parsed::<u32>(span).map(|port| Self { port })
    }
}

#[derive(PartialEq, Debug)]
pub struct Plate {
    pub mix: f32,
}

impl Node<'_> for Plate {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Plate]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        pairs.next_parsed(span).map(|mix| Self { mix })
    }
}

#[derive(PartialEq, Debug)]
pub struct Seq<'ast> {
    pub events: Vec<(f32, UsizeOrRef<&'ast str>)>,
}

impl<'ast> Node<'ast> for Seq<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Seq]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let positives = [Rule::integer, Rule::rest, Rule::note_ref];

        let end_span = span.as_end_span();
        let paras = pairs
            .next()
            .ok_or_else(|| end_span.to_err_with_positives(positives))?;

        // to do, more than a symbol
        // should be an event that contains time and note
        let compounds = paras.into_inner();
        let compounds_num = compounds.len();

        let events = compounds
            .enumerate()
            .map(|(i, compound)| {
                let elements = compound.into_inner();
                let elements_n = elements.len();
                let relative_time_base = i as f32 / compounds_num as f32;

                elements
                    .enumerate()
                    .map(|(j, element)| {
                        let relative_time_sub = j as f32 / elements_n as f32  / compounds_num as f32;
                        let e_span = element.as_end_span();
                        let e = element
                            .into_inner()
                            .next()
                            .ok_or_else(|| e_span.to_err_with_positives(positives))?;

                        let time = relative_time_sub + relative_time_base as f32;

                        match_or_return_err!(e,
                            Rule::integer => {
                                e.try_to_parse()
                                    .map(|num| Some((time, UsizeOrRef::Usize(num))))
                            },
                            Rule::rest => {
                                Ok(None)
                            },
                            Rule::note_ref => {
                                Ok(Some((time, UsizeOrRef::Ref(e.as_str()))))
                            },
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(|elems| elems.into_iter().flatten())
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        
        Ok(Self { events })
    }
}

#[derive(PartialEq, Debug)]
pub struct Choose {
    pub choices: Vec<f32>,
}

impl Node<'_> for Choose {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Choose]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        _s: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        Ok(Self {
            choices: pairs
                .map(|n| n.try_to_parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Arrange<'ast> {
    pub events: Vec<NumberOrRef<&'ast str>>,
}

impl<'ast> Node<'ast> for Arrange<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Arrange]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        _s: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        pairs
            .map(NumberOrRef::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(|events| Self { events })
    }
}

#[derive(PartialEq, Debug)]
pub struct Mix<'ast> {
    pub nodes: Vec<&'ast str>,
}

impl<'ast> Node<'ast> for Mix<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Mix]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        _s: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        Ok(Self {
            nodes: pairs.map(|p| p.as_str()).collect(),
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Sp<'ast> {
    pub sample_sym: &'ast str,
}

impl<'ast> Node<'ast> for Sp<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Sp]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        pairs
            .next()
            .ok_or_else(|| span.as_end_span().to_err_with_positives([Rule::symbol]))
            .map(|sym| Self {
                sample_sym: sym.as_str(),
            })
    }
}

#[derive(PartialEq, Debug)]
pub enum EventValue<'ast> {
    Symbol(&'ast str),
    Number(f32),
}

#[derive(PartialEq, Debug)]
pub struct EventInner<'ast> {
    pub val_times: Vec<(EventValue<'ast>, f32)>,
}

impl<'ast> Node<'ast> for EventInner<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ EventInner]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        pairs
            .next()
            .ok_or_else(|| {
                span.as_end_span()
                    .to_err_with_positives([Rule::pattern_event_body])
            })?
            .into_inner()
            .map(|pair| {
                let end_span = pair.as_end_span();
                let mut iter = pair.into_inner();

                let value_pair = iter
                    .next()
                    .ok_or_else(|| end_span.to_err_with_positives([Rule::value_time]))?;
                let value = match_or_return_err!(value_pair,
                    Rule::number => {
                        EventValue::Number(value_pair.try_to_parse()?)
                    },
                    Rule::symbol => {
                        EventValue::Symbol(value_pair.as_str())
                    },
                );

                let time = iter.next_parsed(end_span)?;

                Ok((value, time))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|val_times| Self { val_times })
    }
}

#[derive(PartialEq, Debug)]
pub struct Pattern<'ast> {
    pub event: EventInner<'ast>,
    pub span: f32,
}

impl<'ast> Node<'ast> for Pattern<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Pattern]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let event = EventInner::parse_from_iter(pairs, span)?;

        let span = pairs.next().map_or(Ok(1.), |r| r.try_to_parse())?;

        Ok(Self { event, span })
    }
}

#[derive(PartialEq, Debug)]
pub struct ConstSig {
    pub value: f32,
}

impl Node<'_> for ConstSig {
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        pairs.next_parsed(span).map(|value| Self { value })
    }
}

#[derive(PartialEq, Debug)]
pub enum Signal<'ast> {
    Number(f32),
    Reference(&'ast str),
    Event(EventInner<'ast>),
    Pattern(Pattern<'ast>),
}

impl<'ast> Node<'ast> for Signal<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Signal]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let paras = pairs.next().ok_or_else(|| {
            span.as_end_span().to_err_with_positives([
                Rule::number,
                Rule::reference,
                Rule::event,
                Rule::pattern,
            ])
        })?;

        match_or_return_err!(paras,
            Rule::number => {
                paras.try_to_parse()
                    .map(Self::Number)
            },
            Rule::reference => {
                Ok(Self::Reference(paras.as_str()))
            },
            Rule::event => {
                EventInner::parse(paras).map(Self::Event)
            },
            Rule::pattern => {
                Pattern::parse(paras).map(Self::Pattern)
            },
        )
    }
}

fn parse_to_two_nums(
    pairs: &mut Pairs<'_, Rule>,
    span: Span<'_>,
) -> Result<[f32; 2], Box<Error<Rule>>> {
    Ok([pairs.next_parsed(span)?, pairs.next_parsed(span)?])
}

#[derive(PartialEq, Debug)]
pub struct SawSynth {
    pub attack: f32,
    pub decay: f32,
}

impl Node<'_> for SawSynth {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ SawSynth]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        parse_to_two_nums(pairs, span).map(|[attack, decay]| Self { attack, decay })
    }
}

#[derive(PartialEq, Debug)]
pub struct SquSynth {
    pub attack: f32,
    pub decay: f32,
}

impl Node<'_> for SquSynth {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ SquSynth]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        parse_to_two_nums(pairs, span).map(|[attack, decay]| Self { attack, decay })
    }
}

#[derive(PartialEq, Debug)]
pub struct TriSynth {
    pub attack: f32,
    pub decay: f32,
}

impl Node<'_> for TriSynth {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ TriSynth]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        parse_to_two_nums(pairs, span).map(|[attack, decay]| Self { attack, decay })
    }
}

#[derive(PartialEq, Debug)]
pub struct MsgSynth<'ast> {
    pub symbol: &'ast str,
    pub attack: f32,
    pub decay: f32,
}

impl<'ast> Node<'ast> for MsgSynth<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ MsgSynth]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();
        let symbol = pairs
            .next()
            .ok_or_else(|| end_span.to_err_with_positives([Rule::symbol]))?
            .as_str();

        let attack = pairs.next_parsed(end_span)?;
        let decay = pairs.next_parsed(end_span)?;

        Ok(Self {
            symbol,
            attack,
            decay,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct PatternSynth<'ast> {
    pub symbol: &'ast str,
    pub span: f32,
}

impl<'ast> Node<'ast> for PatternSynth<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ PatternSynth]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();
        let symbol = pairs
            .next()
            .ok_or_else(|| end_span.to_err_with_positives([Rule::symbol]))?
            .as_str();

        let span = pairs.next_parsed(end_span)?;
        Ok(Self { symbol, span })
    }
}

#[derive(PartialEq, Debug)]
pub struct Lpf<'ast> {
    pub signal: Signal<'ast>,
    pub qvalue: f32,
}

impl<'ast> Node<'ast> for Lpf<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Lpf]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();

        let signal = Signal::parse_from_iter(pairs, span)?;
        let qvalue = pairs.next_parsed(end_span)?;

        Ok(Self { signal, qvalue })
    }
}

#[derive(PartialEq, Debug)]
pub enum PSampler<'ast> {
    Event(EventInner<'ast>),
    Pattern(Pattern<'ast>),
}

impl<'ast> Node<'ast> for PSampler<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ PSampler]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let paras = pairs.next().ok_or_else(|| {
            span.as_end_span()
                .to_err_with_positives([Rule::event, Rule::pattern])
        })?;

        match_or_return_err!(paras,
            Rule::event => {
                EventInner::parse(paras).map(Self::Event)
            },
            Rule::pattern => {
                Pattern::parse(paras).map(Self::Pattern)
            },
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct Balance<'ast> {
    pub left: &'ast str,
    pub right: &'ast str,
}

impl<'ast> Node<'ast> for Balance<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Balance]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();

        Ok(Self {
            left: pairs
                .next()
                .ok_or_else(|| end_span.to_err_with_positives([Rule::reference]))?
                .as_str(),
            right: pairs
                .next()
                .ok_or_else(|| end_span.to_err_with_positives([Rule::reference]))?
                .as_str(),
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Rhpf<'ast> {
    pub cutoff: NumberOrRef<&'ast str>,
    pub qvalue: f32,
}

impl<'ast> Node<'ast> for Rhpf<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Rhpf]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();
        let cutoff = NumberOrRef::parse_from_iter(pairs, span)?;
        let qvalue = pairs.next_parsed(end_span)?;
        Ok(Self { cutoff, qvalue })
    }
}

#[derive(PartialEq, Debug)]
pub struct ApfmsGain<'ast> {
    pub delay: NumberOrRef<&'ast str>,
    pub gain: f32,
}

impl<'ast> Node<'ast> for ApfmsGain<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ ApfmsGain]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();
        let delay = NumberOrRef::parse_from_iter(pairs, span)?;
        let gain = pairs.next_parsed(end_span)?;
        Ok(Self { delay, gain })
    }
}

fn get_f32_arr<const N: usize>(
    pairs: &mut Pairs<'_, Rule>,
    span: Span<'_>,
) -> Result<[f32; N], Box<Error<Rule>>> {
    use std::mem::MaybeUninit;

    let end_span = span.as_end_span();

    // SAFETY: This is safe because we are initializing a bunch of MaybeUninits, which are
    // expected to not be fully initialized, so them being in a uninitialized state is fine.
    // https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
    let mut array: [MaybeUninit<f32>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    let mut initialized = 0;
    let mut err = None;

    for item in &mut array {
        match pairs.next_parsed(end_span) {
            Ok(f) => _ = item.write(f),
            Err(e) => {
                err = Some(e);
                break;
            }
        }
        initialized += 1;
    }

    // MaybeUninit does nothing when dropped, so if it turns out that we need to return an error in
    // the middle of this fn, we can't `?` it - we need to temporarily store it, then drop
    // everything that we initialized so that we don't get a memory leak. Then we can return the
    // error
    if let Some(err) = err {
        for item in array.iter_mut().take(initialized) {
            // SAFETY: This is safe because it's ensured to be initialized, as we initialize them
            // all sequentially, starting at 0, and as soon as something fails, we short-circuit,
            // so everything up to, but not including, `initialized`, must be initialized
            unsafe {
                item.assume_init_drop();
            }
        }
        return Err(err);
    }

    // SAFETY: We must ensure that we actually wrote every item in the array, which did happen
    // because we iterated 0..N and wrote everything
    Ok(array.map(|t| unsafe { t.assume_init() }))
}

#[derive(PartialEq, Debug)]
pub struct Reverb {
    pub dampening: f32,
    pub room_size: f32,
    pub width: f32,
    pub wet: f32,
    pub dry: f32,
}

impl Node<'_> for Reverb {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Reverb]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let [dampening, room_size, width, wet, dry] = get_f32_arr(pairs, span)?;
        Ok(Self {
            dampening,
            room_size,
            width,
            wet,
            dry,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct EnvPerc {
    pub attack: f32,
    pub decay: f32,
}

impl Node<'_> for EnvPerc {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ EnvPerc]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let [attack, decay] = get_f32_arr(pairs, span)?;
        Ok(Self { attack, decay })
    }
}

#[derive(PartialEq, Debug)]
pub struct Adsr {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
}

impl Node<'_> for Adsr {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ Adsr]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'_, Rule>,
        span: Span<'_>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let [attack, decay, sustain, release] = get_f32_arr(pairs, span)?;
        Ok(Self {
            attack,
            decay,
            sustain,
            release,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct CodeBlock<'ast> {
    pub code: &'ast str,
}

impl<'ast> Node<'ast> for CodeBlock<'ast> {
    #[cfg_attr(test, trace::trace(prefix_enter = "[+ CodeBlock]"))]
    fn parse_from_iter(
        pairs: &mut Pairs<'ast, Rule>,
        span: Span<'ast>,
    ) -> Result<Self, Box<Error<Rule>>> {
        let s = pairs
            .next()
            .ok_or_else(|| span.as_end_span().to_err_with_positives([Rule::code]))?
            .as_str();

        Ok(Self {
            code: &s[1..s.len() - 1],
        })
    }
}
