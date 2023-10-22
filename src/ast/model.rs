use std::cell::RefCell;
use crate::ast::availability::Availability;
use crate::ast::comment::Comment;
use crate::ast::decorator::Decorator;
use crate::ast::field::Field;
use crate::ast::handler::HandlerDeclaration;
use crate::ast::identifier::Identifier;
use crate::ast::info_provider::InfoProvider;
use crate::ast::span::Span;

#[derive(Debug)]
pub struct Model {
    pub span: Span,
    pub path: Vec<usize>,
    pub string_path: Vec<String>,
    pub define_availability: Availability,
    pub comment: Option<Comment>,
    pub decorators: Vec<Decorator>,
    pub empty_decorator_spans: Vec<Span>,
    pub identifier: Identifier,
    pub fields: Vec<Field>,
    pub empty_field_decorator_spans: Vec<Span>,
    pub unattached_field_decorators: Vec<Decorator>,
    pub handlers: Vec<HandlerDeclaration>,
    pub resolved: RefCell<Option<ModelResolved>>,
}

impl Model {

    pub fn source_id(&self) -> usize {
        *self.path.first().unwrap()
    }

    pub fn id(&self) -> usize {
        *self.path.last().unwrap()
    }

    pub fn is_available(&self) -> bool {
        self.define_availability.contains(self.resolved().actual_availability)
    }

    pub fn resolve(&self, resolved: ModelResolved) {
        *(unsafe { &mut *self.resolved.as_ptr() }) = Some(resolved);
    }

    pub fn resolved(&self) -> &ModelResolved {
        (unsafe { &*self.resolved.as_ptr() }).as_ref().unwrap()
    }

    pub fn is_resolved(&self) -> bool {
        self.resolved.borrow().is_some()
    }
}

#[derive(Debug)]
pub struct ModelResolved {
    pub scalar_fields: Vec<String>,
    pub scalar_fields_without_virtuals: Vec<String>,
    pub scalar_fields_and_cached_properties_without_virtuals: Vec<String>,
    pub direct_relations: Vec<String>,
    pub relations: Vec<String>,
    pub actual_availability: Availability,
}

impl InfoProvider for &Model {

    fn namespace_str_path(&self) -> Vec<&str> {
        self.string_path.iter().rev().skip(1).rev().map(AsRef::as_ref).collect()
    }

    fn availability(&self) -> Availability {
        self.define_availability.bi_and(self.resolved().actual_availability)
    }
}