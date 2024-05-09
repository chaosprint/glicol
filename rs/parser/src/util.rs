use pest::{error::{Error, ErrorVariant}, iterators::{Pair, Pairs}, RuleType, Span};

use crate::{nodes::{NumberOrRef, TimeList}, Rule};

pub trait ToPestErrWithPositives {
    fn to_err_with_positives<const N: usize, R: RuleType>(self, positives: [R; N]) -> Box<Error<R>>;
}

impl ToPestErrWithPositives for Span<'_> {
    fn to_err_with_positives<const N: usize, R: RuleType>(self, positives: [R; N]) -> Box<Error<R>> {
        Box::new(Error::new_from_span(ErrorVariant::ParsingError {
            positives: positives.to_vec(),
            negatives: vec![]
        }, self))
    }
}

pub trait RuleRepresentable: std::str::FromStr {
    const RULE: Rule;
}

impl RuleRepresentable for f32 {
    const RULE: Rule = Rule::number;
}

impl RuleRepresentable for usize {
    const RULE: Rule = Rule::integer;
}


pub trait TryToParse {
    fn try_to_parse<T>(&self) -> Result<T, Box<Error<Rule>>> where T: RuleRepresentable;
}

impl TryToParse for Pair<'_, Rule> {
    fn try_to_parse<T>(&self) -> Result<T, Box<Error<Rule>>> where T: RuleRepresentable {
        self.as_str()
            .parse::<T>()
            .map_err(|_| self.as_span()
                .to_err_with_positives([T::RULE])
            )
    }
}

pub trait GetNextParsed {
    fn next_parsed<T>(
        &mut self,
        start_span: Span<'_>
    ) -> Result<T, Box<Error<Rule>>>
    where
        T: RuleRepresentable;
}

impl GetNextParsed for Pairs<'_, Rule> {
    fn next_parsed<T>(
        &mut self,
        start_span: Span<'_>
    ) -> Result<T, Box<Error<Rule>>>
    where
        T: RuleRepresentable
    {
        self.next()
            .ok_or_else(|| start_span.to_err_with_positives([T::RULE]))
            .and_then(|p| p.try_to_parse())
    }
}

pub trait EndSpan<'ast> {
    fn as_end_span(&self) -> Span<'ast>;
}

impl<'ast, R> EndSpan<'ast> for Pair<'ast, R> where R: pest::RuleType {
    fn as_end_span(&self) -> Span<'ast> {
        self.as_span().as_end_span()
    }
}

impl<'ast> EndSpan<'ast> for Span<'ast> {
    fn as_end_span(&self) -> Span<'ast> {
        // This is safe to unwrap 'cause we know it's valid due to the indexes we pass in
        self.get(self.end() - self.start()..).unwrap()
    }
}

pub trait ToInnerOwned {
    type Owned;
    fn to_inner_owned(&self) -> Self::Owned;
}

impl<T> ToInnerOwned for NumberOrRef<&T>
where
    T: AsRef<str> + ToOwned + ?Sized,
    <T as ToOwned>::Owned: AsRef<str>
{
    type Owned = NumberOrRef<<T as ToOwned>::Owned>;
    fn to_inner_owned(&self) -> Self::Owned {
        match self {
            Self::Ref(s) => NumberOrRef::Ref((*s).to_owned()),
            Self::Number(n) => NumberOrRef::Number(*n)
        }
    }
}

impl<T, U> ToInnerOwned for Vec<(T, U)>
where
    T: ToInnerOwned,
    U: ToInnerOwned
{
    type Owned = Vec<(<T as ToInnerOwned>::Owned, <U as ToInnerOwned>::Owned)>;
    fn to_inner_owned(&self) -> Self::Owned {
        self.iter()
            .map(|(t, u)| (t.to_inner_owned(), u.to_inner_owned()))
            .collect()
    }
}

impl<T> ToInnerOwned for Vec<T>
where
    T: ToInnerOwned
{
    type Owned = Vec<<T as ToInnerOwned>::Owned>;
    fn to_inner_owned(&self) -> Self::Owned {
        self.iter()
            .map(ToInnerOwned::to_inner_owned)
            .collect()
    }
}

impl ToInnerOwned for f32 {
    type Owned = f32;
    fn to_inner_owned(&self) -> Self::Owned {
        *self
    }
}

impl ToInnerOwned for TimeList {
    type Owned = TimeList;
    fn to_inner_owned(&self) -> Self::Owned {
        self.clone()
    }
}

#[macro_export]
macro_rules! match_or_return_err{
    ($pair:ident, $($variant:path => $arm:tt,)+) => {
        match $pair.as_rule() {
            $($variant => $arm),*
            _ => return ::core::result::Result::Err(::std::boxed::Box::new(::pest::error::Error::new_from_span(
                ::pest::error::ErrorVariant::ParsingError {
                    positives: vec![$($variant,)*],
                    negatives: vec![]
                },
                $pair.as_span()
            )))
        }
    }
}
