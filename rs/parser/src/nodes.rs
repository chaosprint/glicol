use pest::{error::Error, iterators::{Pairs, Pair}, Span};
use hashbrown::HashMap;

use crate::{match_or_return_err, util::{ToPestErrWithPositives, TryToParse, EndSpan, GetNextParsed}, Rule};

#[derive(yoke::Yokeable)]
pub struct Ast<'ast> {
    pub nodes: HashMap<&'ast str, Vec<Component<'ast>>>
}

pub trait Node<'ast> where Self: Sized {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>>;
    fn parse(pair: Pair<'ast, Rule>) -> Result<Self, Box<Error<Rule>>> {
        let span = pair.as_span();
        Self::parse_from_iter(&mut pair.into_inner(), span)
    }
}

pub trait SingleNodeItem<'ast> {
    type Item: Node<'ast>;
    fn from_item(item: Self::Item) -> Self;
}

impl<'ast, T> Node<'ast> for T where T: SingleNodeItem<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        Ok(Self::from_item(T::Item::parse_from_iter(pairs, span)?))
    }
}

// TODO: Make spans more accurate.
// In many cases when we use `as_span().to_err_with_positives()`, we are using a span that's like
// close to where the error is actually occuring, but it would be more accurate to, for example,
// point to the very end or beginning of that span, or some point in the middle.
//
// TODO: Get rid of all unwraps
// We should be able to propogate everything as some sort of pest::error::Error, ideally

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
    Speed(Speed<'ast>),
    ConstSig(ConstSig<'ast>),
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
        fn get_refs<'a, 'ast: 'a, I>(iter: I) -> Vec<&'ast str>
        where
            I: Iterator<Item = &'a NumberOrRef<&'ast str>>
        {
            iter.flat_map(|n| match n {
                NumberOrRef::Number(_) => None,
                NumberOrRef::Ref(r) => Some(*r)
            }).collect()
        }

        match self {
            Self::Delayn(Delayn { param: UsizeOrRef::Ref(r) })
            | Self::Delayms(Delayms { param: NumberOrRef::Ref(r) })
            | Self::Imp(Imp { param: NumberOrRef::Ref(r) })
            | Self::Tri(Tri { param: NumberOrRef::Ref(r) })
            | Self::Squ(Squ { param: NumberOrRef::Ref(r) })
            | Self::Saw(Saw { param: NumberOrRef::Ref(r) })
            | Self::Onepole(Onepole { param: NumberOrRef::Ref(r) })
            | Self::Sin(Sin { param: NumberOrRef::Ref(r) })
            | Self::Mul(Mul { param: NumberOrRef::Ref(r) })
            | Self::Add(Add { param: NumberOrRef::Ref(r) })
            | Self::Pan(Pan { param: NumberOrRef::Ref(r) })
            | Self::Speed(Speed { param: NumberOrRef::Ref(r) })
            | Self::Bd(Bd { param: NumberOrRef::Ref(r) })
            | Self::Sn(Sn { param: NumberOrRef::Ref(r) })
            | Self::Hh(Hh { param: NumberOrRef::Ref(r) })
            | Self::ConstSig(ConstSig::Reference(r))
            | Self::Lpf(Lpf { signal: ConstSig::Reference(r), qvalue: _ })
            | Self::Rhpf(Rhpf { cutoff: NumberOrRef::Ref(r), qvalue: _ })
            | Self::ApfmsGain(ApfmsGain { delay: NumberOrRef::Ref(r), gain: _ })
            | Self::Get(Get { reference: r }) => vec![r],

            Self::Seq(Seq { events }) => get_refs(events.iter().map(|n| &n.1)),
            Self::Arrange(Arrange { events }) => get_refs(events.iter()),
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
    Milliseconds(f32)
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub struct TimeList {
    pub times: Vec<Duration>,
}

#[derive(PartialEq, Debug)]
pub struct Points {
    pub points: Vec<(TimeList, f32)>,
    pub span: f32,
    pub is_looping: bool
}

impl Node<'_> for Points {
    fn parse_from_iter(pairs: &mut Pairs<'_, Rule>, span: Span<'_>) -> Result<Self, Box<Error<Rule>>> {
        let mut node_span = -1.0;
        let mut is_looping = false;
        let points = pairs.next()
            .ok_or_else(|| span.as_end_span().to_err_with_positives([Rule::points_inner]))?;

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

        let points = points.into_inner().map(|point| {
            let end_span = point.as_end_span();
            let mut point_inner = point.into_inner();

            let time = point_inner.next().unwrap();
            let time_end = time.as_end_span();
            let mut time_inner = time.into_inner();
            let bar = time_inner.next().unwrap();

            let mut times = match_or_return_err!(bar,
                Rule::number => {
                    vec![Duration::Bar(bar.try_to_parse()?)]
                },
                Rule::bar => {
                    let mut nums = bar
                        .as_str()
                        .split('/')
                        .map(|x| x.parse::<f32>().unwrap());

                    let num = nums.next().unwrap() / nums.next().unwrap();
                    vec![Duration::Bar(num)]
                },
            );

            if let Some(sign_rule) = time_inner.next() {
                let sign = if sign_rule.as_str() == "-" {
                    -1.0
                } else {
                    1.0
                };

                let s = time_inner.next()
                    .ok_or_else(|| time_end.to_err_with_positives([Rule::second, Rule::ms]))?;
                match_or_return_err!(s,
                    Rule::second => {
                        times.push(Duration::Seconds(
                            sign * s
                                .as_str()
                                .replace("_s", "")
                                .parse::<f32>()
                                .map_err(|_| s.as_span()
                                    .to_err_with_positives([Rule::float])
                                )?
                        ))
                    },
                    Rule::ms => {
                        times.push(Duration::Milliseconds(
                            sign * s
                                .as_str()
                                .replace("_ms", "")
                                .parse::<f32>()
                                .map_err(|_| s.as_span()
                                    .to_err_with_positives([Rule::float])
                                )?
                        ))
                    },
                );
            };

            let value = point_inner.next_parsed(end_span)?;

            Ok((TimeList { times }, value))
        }).collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            points,
            span: node_span,
            is_looping
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NumberOrRef<S>
where
    S: AsRef<str>
{
    Number(f32),
    Ref(S)
}

impl<'ast> Node<'ast> for NumberOrRef<&'ast str> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let paras = pairs.next()
            .ok_or(span.to_err_with_positives([Rule::number, Rule::reference]))?;

        match_or_return_err!(paras,
            Rule::number => {
                Ok(NumberOrRef::Number(paras.try_to_parse()?))
            },
            Rule::reference => {
                Ok(NumberOrRef::Ref(paras.as_str()))
            },
        )
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
    ) => param: UsizeOrRef<'ast>,
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
        Speed,
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
pub struct Get<'ast> {
    pub reference: &'ast str
}

impl<'ast> Node<'ast> for Get<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        pairs.next()
            .ok_or_else(|| span.as_end_span().to_err_with_positives([Rule::reference]))
            .map(|p| Self { reference: p.as_str() })
    }
}

#[derive(PartialEq, Debug)]
pub struct Noise {
    pub seed: usize
}

impl Node<'_> for Noise {
    fn parse_from_iter(pairs: &mut Pairs<'_, Rule>, span: Span<'_>) -> Result<Self, Box<Error<Rule>>> {
        pairs.next_parsed(span).map(|seed| Self { seed })
    }
}

#[derive(PartialEq, Debug)]
pub enum UsizeOrRef<'ast> {
    Usize(usize),
    Ref(&'ast str)
}

impl<'ast> Node<'ast> for UsizeOrRef<'ast>{
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let next = pairs.next()
            .ok_or_else(|| span.as_end_span().to_err_with_positives([Rule::integer, Rule::reference]))?;

        match_or_return_err!(next,
            Rule::number => {
                next.as_str()
                    .parse::<usize>()
                    .map_err(|_| next.as_span().to_err_with_positives([Rule::integer]))
                    .map(Self::Usize)
            },
            Rule::reference => {
                Ok(Self::Ref(next.as_str()))
            },
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct Adc {
    pub port: u32
}

impl Node<'_> for Adc {
    fn parse_from_iter(pairs: &mut Pairs<'_, Rule>, span: Span<'_>) -> Result<Self, Box<Error<Rule>>> {
        pairs.next()
            .and_then(|p| p.as_str().parse::<u32>().ok())
            .ok_or_else(|| span.to_err_with_positives([Rule::integer]))
            .map(|port| Self { port })
    }
}

#[derive(PartialEq, Debug)]
pub struct Plate {
    pub mix: f32
}

impl Node<'_> for Plate {
    fn parse_from_iter(pairs: &mut Pairs<'_, Rule>, span: Span<'_>) -> Result<Self, Box<Error<Rule>>> {
        pairs.next_parsed(span).map(|mix| Self { mix })
    }
}

#[derive(PartialEq, Debug)]
pub struct Seq<'ast> {
    pub events: Vec<(f32, NumberOrRef<&'ast str>)>
}

impl<'ast> Node<'ast> for Seq<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let positives = [
            Rule::integer,
            Rule::rest,
            Rule::note_ref
        ];

        let end_span = span.as_end_span();
        let paras = pairs.next()
            .ok_or_else(|| end_span.to_err_with_positives(positives))?;

        // to do, more than a symbol
        // should be an event that contains time and note
        // GlicolPara::Symbol(paras.as_str())
        let compounds = paras.into_inner();
        // one bar will firstly be divided here
        let compounds_num = compounds.len();

        let events = compounds.enumerate().map(|(i, compound)| {
            let relative_time_base =
                i as f32 / compounds_num as f32;
            let elements = compound.into_inner();
            let elements_n = elements.len();

            elements.enumerate().map(|(j, element)| {
                let relative_time_sub = 1. / compounds_num as f32
                    * j as f32
                    / elements_n as f32;
                let e_span = element.as_end_span();
                let e = element.into_inner()
                    .next()
                    .ok_or_else(|| e_span.to_err_with_positives(positives))?;

                let time = relative_time_sub + relative_time_base;

                match_or_return_err!(e,
                    Rule::integer => {
                        e.try_to_parse()
                            .map(|num| Some((time, NumberOrRef::Number(num))))
                    },
                    Rule::rest => {
                        Ok(None)
                    },
                    Rule::note_ref => {
                        Ok(Some((time, NumberOrRef::Ref(e.as_str()))))
                    },
                )
            }).collect::<Result<Vec<_>, _>>()
            .map(|elems| elems.into_iter().flatten())

        }).collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();

        Ok(Self { events })
    }
}

#[derive(PartialEq, Debug)]
pub struct Choose {
    pub choices: Vec<f32>
}

impl Node<'_> for Choose {
    fn parse_from_iter(pairs: &mut Pairs<'_, Rule>, _: Span<'_>) -> Result<Self, Box<Error<Rule>>> {
        Ok(Self {
            choices: pairs
                .map(|n| n.try_to_parse())
                .collect::<Result<Vec<_>, _>>()?
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Arrange<'ast> {
    pub events: Vec<NumberOrRef<&'ast str>>
}

impl<'ast> Node<'ast> for Arrange<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, _: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        Ok(Self {
            events: pairs
                .map(NumberOrRef::parse)
                .collect::<Result<Vec<_>, _>>()?
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Mix<'ast> {
    pub nodes: Vec<&'ast str>
}

impl<'ast> Node<'ast> for Mix<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, _: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        Ok(Self {
            nodes: pairs
                .map(|p| p.as_str())
                .collect()
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Sp<'ast> {
    pub sample_sym: &'ast str
}

impl<'ast> Node<'ast> for Sp<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        Ok(Self {
            sample_sym: pairs.next()
                // TODO: Reference isn't the right type of error for right here, but the grammar
                // currently doesn't contain something for this. Should we modify the grammar to
                // make this more specific? Probably
                .ok_or_else(|| span.to_err_with_positives([Rule::reference]))?
                .as_str()
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum EventValue<'ast> {
    Symbol(&'ast str),
    Number(f32)
}

#[derive(PartialEq, Debug)]
pub struct EventInner<'ast> {
    pub val_times: Vec<(EventValue<'ast>, f32)>
}

impl<'ast> Node<'ast> for EventInner<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        pairs.next()
            .ok_or_else(|| span.to_err_with_positives([Rule::pattern_event_body]))?
            .into_inner()
            .map(|pair| {
                let end_span = pair.as_end_span();
                let mut iter = pair.into_inner();

                let value_pair = iter.next()
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
    pub span: f32
}

impl<'ast> Node<'ast> for Pattern<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let event = EventInner::parse_from_iter(pairs, span)?;

        let span = pairs.next()
            .map_or(Ok(1.), |r| r.try_to_parse())?;

        Ok(Self { event, span })
    }
}

#[derive(PartialEq, Debug)]
pub enum ConstSig<'ast> {
    Number(f32),
    Reference(&'ast str),
    Event(EventInner<'ast>),
    Pattern(Pattern<'ast>)
}

impl<'ast> Node<'ast> for ConstSig<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let paras = pairs.next()
            .ok_or_else(|| span.to_err_with_positives([
                Rule::number,
                Rule::reference,
                Rule::event,
                Rule::pattern
            ]))?;

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

fn parse_to_two_nums(pairs: &mut Pairs<'_, Rule>, span: Span<'_>) -> Result<[f32; 2], Box<Error<Rule>>> {
    Ok([pairs.next_parsed(span)?, pairs.next_parsed(span)?])
}

#[derive(PartialEq, Debug)]
pub struct SawSynth {
    pub attack: f32,
    pub decay: f32
}

impl Node<'_> for SawSynth {
    fn parse_from_iter(pairs: &mut Pairs<'_, Rule>, span: Span<'_>) -> Result<Self, Box<Error<Rule>>> {
        parse_to_two_nums(pairs, span)
            .map(|[attack, decay]| Self { attack, decay })
    }
}

#[derive(PartialEq, Debug)]
pub struct SquSynth {
    pub attack: f32,
    pub decay: f32
}

impl Node<'_> for SquSynth {
    fn parse_from_iter(pairs: &mut Pairs<'_, Rule>, span: Span<'_>) -> Result<Self, Box<Error<Rule>>> {
        parse_to_two_nums(pairs, span)
            .map(|[attack, decay]| Self { attack, decay })
    }
}

#[derive(PartialEq, Debug)]
pub struct TriSynth {
    pub attack: f32,
    pub decay: f32
}

impl Node<'_> for TriSynth {
    fn parse_from_iter(pairs: &mut Pairs<'_, Rule>, span: Span<'_>) -> Result<Self, Box<Error<Rule>>> {
        parse_to_two_nums(pairs, span)
            .map(|[attack, decay]| Self { attack, decay })
    }
}

#[derive(PartialEq, Debug)]
pub struct MsgSynth<'ast> {
    pub symbol: &'ast str,
    pub attack: f32,
    pub decay: f32,
}

impl<'ast> Node<'ast> for MsgSynth<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();
        let symbol = pairs.next()
            .ok_or_else(|| end_span.to_err_with_positives([Rule::symbol]))?
            .as_str();

        let attack = pairs.next_parsed(end_span)?;
        let decay = pairs.next_parsed(end_span)?;

        Ok(Self { symbol, attack, decay })
    }
}

#[derive(PartialEq, Debug)]
pub struct PatternSynth<'ast> {
    pub symbol: &'ast str,
    // TODO: This isn't named in the grammar so i don't know what to call it
    pub p2: f32
}

impl<'ast> Node<'ast> for PatternSynth<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();
        let symbol = pairs.next()
            .ok_or_else(|| end_span.to_err_with_positives([Rule::symbol]))?
            .as_str();

        let p2 = pairs.next_parsed(end_span)?;
        Ok(Self { symbol, p2 })
    }
}

#[derive(PartialEq, Debug)]
pub struct Lpf<'ast> {
    pub signal: ConstSig<'ast>,
    pub qvalue: f32
}

impl<'ast> Node<'ast> for Lpf<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();

        let signal = ConstSig::parse_from_iter(pairs, span)?;
        let qvalue = pairs.next_parsed(end_span)?;

        Ok(Self { signal, qvalue })
    }
}

#[derive(PartialEq, Debug)]
pub enum PSampler<'ast> {
    Event(EventInner<'ast>),
    Pattern(Pattern<'ast>)
}

impl<'ast> Node<'ast> for PSampler<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let paras = pairs.next()
            .ok_or_else(|| span.to_err_with_positives([Rule::event, Rule::pattern]))?;

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
    pub right: &'ast str
}

impl<'ast> Node<'ast> for Balance<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();

        Ok(Self {
            left: pairs.next()
                .ok_or_else(|| end_span.to_err_with_positives([Rule::reference]))?
                .as_str(),
            right: pairs.next()
                .ok_or_else(|| end_span.to_err_with_positives([Rule::reference]))?
                .as_str()
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Rhpf<'ast> {
    pub cutoff: NumberOrRef<&'ast str>,
    pub qvalue: f32
}

impl<'ast> Node<'ast> for Rhpf<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();
        let cutoff = NumberOrRef::parse_from_iter(pairs, span)?;
        let qvalue = pairs.next_parsed(end_span)?;
        Ok(Self { cutoff, qvalue })
    }
}

#[derive(PartialEq, Debug)]
pub struct ApfmsGain<'ast> {
    pub delay: NumberOrRef<&'ast str>,
    pub gain: f32
}

impl<'ast> Node<'ast> for ApfmsGain<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let end_span = span.as_end_span();
        let delay = NumberOrRef::parse_from_iter(pairs, span)?;
        let gain = pairs.next_parsed(end_span)?;
        Ok(Self { delay, gain })
    }
}

fn get_f32_arr<const N: usize>(
    pairs: &mut Pairs<'_, Rule>,
    span: Span<'_>
) -> Result<[f32; N], Box<Error<Rule>>> {
    use std::mem::MaybeUninit;

    let end_span = span.as_end_span();

    // SAFETY: This is safe because we are initializing a bunch of MaybeUninits, which are
    // expected to not be fully initialized, so them being in a uninitialized state is fine.
    // https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
    let mut array: [MaybeUninit<f32>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    let mut initialized = 0;
    let mut err = None;
    //for i in 0..N {
    for item in &mut array {
        match pairs.next()
            .ok_or(end_span)
            .and_then(|p| p.as_str().parse::<f32>().map_err(|_| p.as_span()))
            .map_err(|span| span.to_err_with_positives([Rule::float]))
        {
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
            unsafe { item.assume_init_drop(); }
        }
        return Err(err);
    }

    // SAFETY: We must ensure that we actually wrote every item in the array, which did happen
    // because we iterated 0..N and wrote everything
    Ok(array.map(|t| unsafe { t.assume_init() }))
}

// TODO: What are the actual param names?
#[derive(PartialEq, Debug)]
pub struct Reverb {
    pub p1: f32,
    pub p2: f32,
    pub p3: f32,
    pub p4: f32,
    pub p5: f32
}

impl Node<'_> for Reverb {
    fn parse_from_iter(pairs: &mut Pairs<'_, Rule>, span: Span<'_>) -> Result<Self, Box<Error<Rule>>> {
        let [p1, p2, p3, p4, p5] = get_f32_arr(pairs, span)?;
        Ok(Self { p1, p2, p3, p4, p5 })
    }
}

#[derive(PartialEq, Debug)]
pub struct EnvPerc {
    pub attack: f32,
    pub decay: f32
}

impl Node<'_> for EnvPerc {
    fn parse_from_iter(pairs: &mut Pairs<'_, Rule>, span: Span<'_>) -> Result<Self, Box<Error<Rule>>> {
        let [attack, decay] = get_f32_arr(pairs, span)?;
        Ok(Self { attack, decay })
    }
}

#[derive(PartialEq, Debug)]
pub struct Adsr {
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32
}

impl Node<'_> for Adsr {
    fn parse_from_iter(pairs: &mut Pairs<'_, Rule>, span: Span<'_>) -> Result<Self, Box<Error<Rule>>> {
        let [attack, decay, sustain, release] = get_f32_arr(pairs, span)?;
        Ok(Self { attack, decay, sustain, release })
    }
}

#[derive(PartialEq, Debug)]
pub struct CodeBlock<'ast> {
    pub code: &'ast str
}

impl<'ast> Node<'ast> for CodeBlock<'ast> {
    fn parse_from_iter(pairs: &mut Pairs<'ast, Rule>, span: Span<'ast>) -> Result<Self, Box<Error<Rule>>> {
        let s = pairs.next()
            .ok_or_else(|| span.to_err_with_positives([Rule::code]))?
            .as_str();

        Ok(Self { code: &s[1..s.len() - 1] })
    }
}
