use std::collections::BTreeMap;
use crate::ast::decorator::{Decorator};
use crate::ast::reference_space::ReferenceSpace;
use crate::r#type::keyword::Keyword;
use crate::r#type::r#type::Type;
use crate::resolver::resolve_argument_list::{resolve_argument_list};
use crate::resolver::resolve_identifier::resolve_identifier_path;
use crate::resolver::resolver_context::ResolverContext;
use crate::traits::node_trait::NodeTrait;
use crate::traits::resolved::Resolve;

pub(super) fn resolve_decorator<'a>(
    decorator: &'a Decorator,
    context: &'a ResolverContext<'a>,
    keywords_map: &BTreeMap<Keyword, Type>,
    reference_type: ReferenceSpace,
) {
    if let Some(reference) = resolve_identifier_path(decorator.identifier_path(), context, reference_type, context.current_availability()) {
        println!("see reference: {:?}", reference);
        decorator.resolve(reference.r#type().as_decorator_reference().unwrap().path().clone());
        let decorator_declaration = context.schema.find_top_by_path(decorator.resolved()).unwrap().as_decorator_declaration().unwrap();
        resolve_argument_list(
            decorator.identifier_path().identifiers().last().unwrap().span(),
            decorator.argument_list(),
            decorator_declaration.callable_variants(),
            keywords_map,
            context,
            None,
        );
    } else {
        context.insert_diagnostics_error(decorator.identifier_path().span(), "decorator not found")
    }
}