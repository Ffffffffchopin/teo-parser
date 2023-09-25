use std::fmt::{Display, Formatter};
use crate::parser::ast::entity::Entity;
use crate::parser::ast::expression::Expression;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct Constant {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: ASTIdentifier,
    pub(crate) expression: Expression,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
}

impl Constant {
    pub(crate) fn new(item_id: usize, source_id: usize, identifier: ASTIdentifier, expression: Expression, span: Span) -> Self {
        Self {
            id: item_id,
            source_id,
            identifier,
            expression,
            span,
            resolved: false,
        }
    }

    pub(crate) fn entity(&self) -> &Entity {
        if !self.resolved {
            panic!("Constant is not resolved while accessed.")
        }
        self.expression.resolved.as_ref().unwrap()
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("let ")?;
        Display::fmt(&self.identifier, f)?;
        f.write_str(" = ")?;
        Display::fmt(&self.expression, f)
    }
}
