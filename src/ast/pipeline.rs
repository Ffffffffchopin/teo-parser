use std::fmt::{Display, Formatter};
use crate::ast::expression::ExpressionKind;
use crate::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct ASTPipeline {
    pub(crate) expression: Box<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for ASTPipeline {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("$")?;
        Display::fmt(&self.expression, f)
    }
}
