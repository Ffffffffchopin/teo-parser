use serde::Serialize;
use crate::ast::doc_comment::DocComment;
use crate::ast::decorator::Decorator;
use crate::ast::type_expr::{TypeExpr};
use crate::ast::identifier::Identifier;
use crate::ast::span::Span;
use crate::{declare_container_node, impl_container_node_defaults, node_child_fn, node_children_iter, node_children_iter_fn, node_optional_child_fn};
use crate::format::Writer;
use crate::traits::has_availability::HasAvailability;
use crate::traits::info_provider::InfoProvider;
use crate::traits::write::Write;

declare_container_node!(HandlerGroupDeclaration, named, availability,
    pub(crate) comment: Option<usize>,
    pub(crate) identifier: usize,
    pub(crate) handler_declarations: Vec<usize>,
);

impl_container_node_defaults!(HandlerGroupDeclaration, named, availability);

node_children_iter!(HandlerGroupDeclaration, HandlerDeclaration, HandlerDeclarationsIter, handler_declarations);

impl HandlerGroupDeclaration {

    node_optional_child_fn!(comment, DocComment);

    node_child_fn!(identifier, Identifier);

    node_children_iter_fn!(handler_declarations, HandlerDeclarationsIter);
}

impl InfoProvider for HandlerGroupDeclaration {
    fn namespace_skip(&self) -> usize {
        1
    }
}

declare_container_node!(HandlerDeclaration, named, availability,
    pub(crate) comment: Option<usize>,
    pub(crate) decorators: Vec<usize>,
    pub(crate) empty_decorators_spans: Vec<Span>,
    pub(crate) identifier: usize,
    pub(crate) input_type: usize,
    pub(crate) output_type: usize,
    pub input_format: HandlerInputFormat,
);

impl_container_node_defaults!(HandlerDeclaration, named, availability);

node_children_iter!(HandlerDeclaration, Decorator, DecoratorsIter, decorators);

impl HandlerDeclaration {

    node_optional_child_fn!(comment, DocComment);

    node_children_iter_fn!(decorators, DecoratorsIter);

    node_child_fn!(identifier, Identifier);

    node_child_fn!(input_type, TypeExpr);

    node_child_fn!(output_type, TypeExpr);
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum HandlerInputFormat {
    Json,
    Form,
}

impl HandlerInputFormat {

    pub fn is_json(&self) -> bool {
        match self {
            HandlerInputFormat::Json => true,
            _ => false,
        }
    }

    pub fn is_form(&self) -> bool {
        match self {
            HandlerInputFormat::Form => true,
            _ => false,
        }
    }
}

impl InfoProvider for HandlerDeclaration {
    fn namespace_skip(&self) -> usize {
        2
    }
}

impl Write for HandlerGroupDeclaration {
    fn write<'a>(&'a self, writer: &'a mut Writer<'a>) {
        writer.write_children(self, self.children.values())
    }

    fn is_block_level_element(&self) -> bool {
        true
    }
}

impl Write for HandlerDeclaration {
    fn write<'a>(&'a self, writer: &'a mut Writer<'a>) {
        writer.write_children(self, self.children.values())
    }

    fn is_block_level_element(&self) -> bool {
        true
    }
}