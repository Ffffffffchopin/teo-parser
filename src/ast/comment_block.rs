use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub struct CommentBlock {
    pub(crate) name: Option<String>,
    pub(crate) desc: Option<String>,
    pub(crate) span: Span,
}

impl CommentBlock {
    pub(crate) fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| n.as_str())
    }

    pub(crate) fn desc(&self) -> Option<&str> {
        self.desc.as_ref().map(|n| n.as_str())
    }
}