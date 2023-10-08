use crate::ast::interface::InterfaceDeclaration;
use crate::ast::struct_declaration::StructDeclaration;
use crate::resolver::resolve_field::{FieldParentType, resolve_field};
use crate::resolver::resolve_function_declaration::resolve_function_declaration;
use crate::resolver::resolve_generics::resolve_generics_declaration;
use crate::resolver::resolve_type_expr::resolve_type_expr;
use crate::resolver::resolver_context::ResolverContext;

pub(super) fn resolve_struct_declaration<'a>(struct_declaration: &'a StructDeclaration, context: &'a ResolverContext<'a>) {
    if context.has_examined_default_path(&struct_declaration.string_path) {
        context.insert_duplicated_identifier(struct_declaration.identifier.span);
    }
    if let Some(generics_declaration) = &struct_declaration.generics_declaration {
        resolve_generics_declaration(generics_declaration, context)
    }

    for function_declaration in &struct_declaration.function_declarations {
        resolve_function_declaration(
            function_declaration,
            struct_declaration.generics_declaration.as_ref(),
            struct_declaration.generics_constraint.as_ref(),
            context,
        )
    }
}
