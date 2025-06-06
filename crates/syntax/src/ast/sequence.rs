use std::slice::Iter;
use std::vec::IntoIter;

use serde::Deserialize;
use serde::Serialize;

use mago_span::HasSpan;
use mago_span::Position;
use mago_span::Span;

use crate::token::Token;

/// Represents a sequence of nodes.
///
/// An example of this is modifiers in a method declaration.
///
/// i.e. `public` and `static` in `public static function foo() {}`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Sequence<T> {
    pub nodes: Vec<T>,
}

/// Represents a sequence of nodes separated by a token.
///
/// An example of this is arguments in a function call, where the tokens are commas.
///
/// i.e. `1`, `2` and `3` in `foo(1, 2, 3)`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct TokenSeparatedSequence<T> {
    pub nodes: Vec<T>,
    pub tokens: Vec<Token>,
}

impl<T: HasSpan> Sequence<T> {
    #[inline]
    pub const fn new(inner: Vec<T>) -> Self {
        Self { nodes: inner }
    }

    #[inline]
    pub const fn empty() -> Self {
        Self { nodes: vec![] }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.nodes.get(index)
    }

    #[inline]
    #[must_use]
    pub fn first(&self) -> Option<&T> {
        self.nodes.first()
    }

    #[inline]
    #[must_use]
    pub fn first_span(&self) -> Option<Span> {
        self.nodes.first().map(|node| node.span())
    }

    #[inline]
    #[must_use]
    pub fn last(&self) -> Option<&T> {
        self.nodes.last()
    }

    #[inline]
    #[must_use]
    pub fn last_span(&self) -> Option<Span> {
        self.nodes.last().map(|node| node.span())
    }

    #[inline]
    #[must_use]
    pub fn span(&self, from: Position) -> Span {
        self.last_span().map_or(Span::new(from, from), |span| Span::new(from, span.end))
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.nodes.iter()
    }

    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.nodes.as_slice()
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<&T> {
        self.nodes.iter().collect()
    }
}

impl<T: HasSpan> TokenSeparatedSequence<T> {
    #[inline]
    #[must_use]
    pub const fn new(inner: Vec<T>, tokens: Vec<Token>) -> Self {
        Self { nodes: inner, tokens }
    }

    #[inline]
    #[must_use]
    pub const fn empty() -> Self {
        Self { nodes: vec![], tokens: vec![] }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.nodes.get(index)
    }

    #[inline]
    #[must_use]
    pub fn first(&self) -> Option<&T> {
        self.nodes.first()
    }

    #[inline]
    pub fn first_span(&self) -> Option<Span> {
        match (self.tokens.first(), self.nodes.first()) {
            (Some(token), Some(node)) => {
                // check if the token comes before the node
                if token.span.end <= node.span().start { Some(token.span) } else { Some(node.span()) }
            }
            (Some(token), None) => Some(token.span),
            (None, Some(node)) => Some(node.span()),
            (None, None) => None,
        }
    }

    #[inline]
    pub fn last(&self) -> Option<&T> {
        self.nodes.last()
    }

    #[inline]
    pub fn last_span(&self) -> Option<Span> {
        match (self.tokens.last(), self.nodes.last()) {
            (Some(token), Some(node)) => {
                // check if the token comes after the node
                if token.span.start >= node.span().end { Some(token.span) } else { Some(node.span()) }
            }
            (Some(token), None) => Some(token.span),
            (None, Some(node)) => Some(node.span()),
            (None, None) => None,
        }
    }

    #[inline]
    pub fn span(&self, from: Position) -> Span {
        match (self.first_span(), self.last_span()) {
            (Some(first), Some(last)) => Span::new(first.start, last.end),
            _ => Span::new(from, from),
        }
    }

    #[inline]
    pub fn has_trailing_token(&self) -> bool {
        self.tokens
            .last()
            .is_some_and(|token| token.span.start.offset >= self.nodes.last().map_or(0, |node| node.span().end.offset))
    }

    #[inline]
    pub fn get_trailing_token(&self) -> Option<&Token> {
        self.tokens
            .last()
            .filter(|token| token.span.start.offset >= self.nodes.last().map_or(0, |node| node.span().end.offset))
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.nodes.iter()
    }

    /// Returns an iterator over the sequence, where each item includes
    /// the index of the element, the element and the token following it.
    /// The token is `None` only for the last element if it has no trailing token.
    #[inline]
    pub fn iter_with_tokens(&self) -> impl Iterator<Item = (usize, &T, Option<&Token>)> {
        self.nodes.iter().enumerate().map(move |(i, item)| {
            let token = self.tokens.get(i);

            (i, item, token)
        })
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.nodes.as_slice()
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<&T> {
        self.nodes.iter().collect()
    }
}

impl<T: HasSpan> FromIterator<T> for Sequence<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self { nodes: iter.into_iter().collect() }
    }
}

impl<T: HasSpan> IntoIterator for Sequence<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl<T: HasSpan> IntoIterator for TokenSeparatedSequence<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl<T: HasSpan> std::default::Default for Sequence<T> {
    fn default() -> Self {
        Sequence::new(Default::default())
    }
}

impl<T: HasSpan> std::default::Default for TokenSeparatedSequence<T> {
    fn default() -> Self {
        TokenSeparatedSequence::new(Default::default(), Default::default())
    }
}
