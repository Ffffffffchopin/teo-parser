use maplit::btreemap;
use crate::ast::interface::InterfaceDeclaration;
use crate::resolver::resolve_field::{FieldParentType, resolve_field_class, resolve_field_types};
use crate::resolver::resolve_generics::{resolve_generics_constraint, resolve_generics_declaration};
use crate::resolver::resolve_type_expr::resolve_type_expr;
use crate::resolver::resolver_context::ResolverContext;
use crate::traits::node_trait::NodeTrait;
use crate::traits::resolved::Resolve;

pub(super) fn resolve_interface_declaration_types<'a>(interface_declaration: &'a InterfaceDeclaration, context: &'a ResolverContext<'a>) {
    if context.has_examined_default_path(&interface_declaration.string_path, interface_declaration.define_availability) {
        context.insert_duplicated_identifier(interface_declaration.identifier().span);
    }
    *interface_declaration.actual_availability.borrow_mut() = context.current_availability();
    if let Some(generics_declaration) = interface_declaration.generics_declaration() {
        resolve_generics_declaration(generics_declaration, &vec![], context);
        if let Some(generics_constraint) = interface_declaration.generics_constraint() {
            resolve_generics_constraint(generics_constraint, context, generics_declaration, interface_declaration.define_availability);
        }
    }
    for extend in interface_declaration.extends() {
        resolve_type_expr(
            extend,
            &if let Some(generics_declaration) = interface_declaration.generics_declaration() {
                vec![generics_declaration]
            } else {
                vec![]
            },
            &if let Some(generics_constraint) = interface_declaration.generics_constraint() {
                vec![generics_constraint]
            } else {
                vec![]
            },
            &btreemap! {},
            context,
            interface_declaration.define_availability,
        );
        if !extend.resolved().is_interface_object() {
            context.insert_diagnostics_error(extend.span(), "TypeError: type is not interface");
        }
    }
    for partial_field in interface_declaration.partial_fields() {
        context.insert_diagnostics_error(partial_field.span, "partial field");
    }
    for field in interface_declaration.fields() {
        resolve_field_class(
            field,
            FieldParentType::Interface,
            context,
        );
        resolve_field_types(
            field,
            interface_declaration.generics_declaration(),
            interface_declaration.generics_constraint(),
            context
        );
    }
    context.add_examined_default_path(interface_declaration.string_path.clone(), interface_declaration.define_availability);
}
