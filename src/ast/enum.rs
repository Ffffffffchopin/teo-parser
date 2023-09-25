use crate::parser::ast::comment_block::CommentBlock;
use crate::parser::ast::decorator::ASTDecorator;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::span::Span;

#[derive(Debug)]
pub(crate) struct ASTEnum {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) id_path: Vec<usize>,
    pub(crate) comment_block: Option<CommentBlock>,
    pub(crate) identifier: ASTIdentifier,
    pub(crate) ns_path: Vec<String>,
    pub(crate) decorators: Vec<ASTDecorator>,
    pub(crate) choices: Vec<EnumChoice>,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
}

impl ASTEnum {
    pub(crate) fn new(item_id: usize, source_id: usize, id_path: Vec<usize>, comment_block: Option<CommentBlock>, identifier: ASTIdentifier, ns_path: Vec<String>, decorators: Vec<ASTDecorator>, choices: Vec<EnumChoice>, span: Span) -> Self {
        Self {
            id: item_id,
            source_id,
            id_path,
            comment_block,
            identifier,
            decorators,
            choices,
            span,
            ns_path,
            resolved: false,
        }
    }

    pub(crate) fn path(&self) -> Vec<String> {
        let mut result = self.ns_path.clone();
        result.push(self.identifier.name.clone());
        result
    }
}

#[derive(Debug)]
pub(crate) struct EnumChoice {
    pub(crate) identifier: ASTIdentifier,
    pub(crate) comment_block: Option<CommentBlock>,
    pub(crate) decorators: Vec<ASTDecorator>,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
}

impl EnumChoice {
    pub(crate) fn new(identifier: ASTIdentifier, comment_block: Option<CommentBlock>, decorators: Vec<ASTDecorator>, span: Span) -> Self {
        Self { identifier, decorators, span, comment_block, resolved: false }
    }
}
