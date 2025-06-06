use serde::Deserialize;
use serde::Serialize;

use mago_span::HasSpan;
use mago_span::Span;

use crate::ast::ast::attribute::AttributeList;
use crate::ast::ast::class_like::property::PropertyHookList;
use crate::ast::ast::expression::Expression;
use crate::ast::ast::modifier::Modifier;
use crate::ast::ast::type_hint::Hint;
use crate::ast::ast::variable::DirectVariable;
use crate::ast::sequence::Sequence;
use crate::ast::sequence::TokenSeparatedSequence;

/// Represents a parameter list in PHP.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[repr(C)]
pub struct FunctionLikeParameterList {
    pub left_parenthesis: Span,
    pub parameters: TokenSeparatedSequence<FunctionLikeParameter>,
    pub right_parenthesis: Span,
}

/// Represents a function-like parameter in PHP.
///
/// Example: `int $foo`, `string &$bar`, `bool ...$baz`, `mixed $qux = null`
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[repr(C)]
pub struct FunctionLikeParameter {
    pub attribute_lists: Sequence<AttributeList>,
    pub modifiers: Sequence<Modifier>,
    pub hint: Option<Hint>,
    pub ampersand: Option<Span>,
    pub ellipsis: Option<Span>,
    pub variable: DirectVariable,
    pub default_value: Option<FunctionLikeParameterDefaultValue>,
    pub hooks: Option<PropertyHookList>,
}

/// Represents the default value of a function-like parameter.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[repr(C)]
pub struct FunctionLikeParameterDefaultValue {
    pub equals: Span,
    pub value: Expression,
}

impl FunctionLikeParameter {
    /// Returns whether the parameter is a promoted property.
    ///
    /// A promoted property is a property that is declared in a constructor parameter list.
    ///
    /// A parameter is considered a promoted property if it has at least one modifier or a hook.
    ///
    /// [RFC: Constructor Property Promotion](https://wiki.php.net/rfc/constructor_promotion)
    /// [RFC: Property Hooks](https://wiki.php.net/rfc/property-hooks)
    pub fn is_promoted_property(&self) -> bool {
        !self.modifiers.is_empty() || self.hooks.is_some()
    }

    #[inline]
    pub const fn is_variadic(&self) -> bool {
        self.ellipsis.is_some()
    }

    #[inline]
    pub const fn is_reference(&self) -> bool {
        self.ampersand.is_some()
    }
}

impl HasSpan for FunctionLikeParameterList {
    fn span(&self) -> Span {
        Span::between(self.left_parenthesis, self.right_parenthesis)
    }
}

impl HasSpan for FunctionLikeParameter {
    fn span(&self) -> Span {
        let right = self.hooks.as_ref().map(|hooks| hooks.span()).unwrap_or_else(|| {
            self.default_value.as_ref().map_or_else(|| self.variable.span(), |default_value| default_value.span())
        });

        if let Some(attribute) = self.attribute_lists.first() {
            return Span::between(attribute.span(), right);
        }

        if let Some(modifier) = self.modifiers.first() {
            return Span::between(modifier.span(), right);
        }

        if let Some(type_hint) = &self.hint {
            return Span::between(type_hint.span(), right);
        }

        if let Some(ellipsis) = self.ellipsis {
            return Span::between(ellipsis, right);
        }

        if let Some(ampersand) = self.ampersand {
            return Span::between(ampersand, right);
        }

        Span::between(self.variable.span(), right)
    }
}

impl HasSpan for FunctionLikeParameterDefaultValue {
    fn span(&self) -> Span {
        Span::between(self.equals, self.value.span())
    }
}
