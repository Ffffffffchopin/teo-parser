use crate::ast::comment::Comment;
use crate::ast::decorator::Decorator;
use crate::ast::identifier::Identifier;
use crate::ast::span::Span;

#[derive(Debug)]
pub(crate) struct Enum {
    pub(crate) path: Vec<usize>,
    pub(crate) comment: Option<Comment>,
    pub(crate) identifier: Identifier,
    pub(crate) string_path: Vec<String>,
    pub(crate) decorators: Vec<Decorator>,
    pub(crate) members: Vec<EnumMember>,
    pub(crate) span: Span,
}

impl Enum {

    pub(crate) fn new(
        path: Vec<usize>,
        string_path: Vec<String>,
        comment: Option<Comment>,
        identifier: Identifier,
        decorators: Vec<Decorator>,
        members: Vec<EnumMember>,
        span: Span
    ) -> Self {
        Self {
            path,
            string_path,
            comment,
            identifier,
            decorators,
            members,
            span,
        }
    }
}

#[derive(Debug)]
pub(crate) struct EnumMember {
    pub(crate) identifier: Identifier,
    pub(crate) comment: Option<Comment>,
    pub(crate) decorators: Vec<Decorator>,
    pub(crate) span: Span,
}

impl EnumMember {

    pub(crate) fn new(
        identifier: Identifier,
        comment: Option<Comment>,
        decorators: Vec<Decorator>,
        span: Span
    ) -> Self {
        Self {
            identifier,
            decorators,
            span,
            comment,
        }
    }
}
