use std::collections::HashMap;
use std::path::PathBuf;
use maplit::hashmap;
use crate::ast::identifier::Identifier;
use crate::ast::literals::StringLiteral;
use crate::ast::reference::Reference;
use crate::ast::span::Span;

#[derive(Clone, Debug)]
pub(crate) struct Import {
    pub(crate) path: Vec<usize>,
    pub(crate) identifiers: Vec<Identifier>,
    pub(crate) source: StringLiteral,
    pub(crate) file_path: PathBuf,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
    pub(crate) from_id: Option<usize>,
    pub(crate) references: HashMap<String, Reference>,
}

impl Import {

    pub(crate) fn new(path: Vec<usize>, identifiers: Vec<Identifier>, source: StringLiteral, file_path: PathBuf, span: Span) -> Self {
        Self {
            path,
            identifiers,
            source,
            file_path,
            span,
            resolved: false,
            from_id: None,
            references: hashmap!{},
        }
    }

    pub(crate) fn source_id(&self) -> usize {
        *self.path.first().unwrap()
    }

    pub(crate) fn id(&self) -> usize {
        *self.path.last().unwrap()
    }
}
