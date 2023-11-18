use std::sync::Arc;
use crate::ast::node::Node;
use crate::availability::Availability;
use crate::value::ExprInfo;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::r#type::reference::Reference;
use crate::r#type::Type;
use crate::traits::identifiable::Identifiable;
use crate::traits::resolved::Resolve;

pub fn search_identifier_path_names_with_filter_to_type_and_value(
    identifier_path_names: &Vec<&str>,
    schema: &Schema,
    source: &Source,
    namespace_str_path: &Vec<&str>,
    filter: &Arc<dyn Fn(&Node) -> bool>,
    availability: Availability,
) -> Option<ExprInfo> {
    search_identifier_path_names_with_filter_to_top(
        identifier_path_names,
        schema,
        source,
        namespace_str_path,
        filter,
        availability
    ).map(|t| top_to_reference_type_and_value(t))
}

pub fn search_identifier_path_names_with_filter_to_path(
    identifier_path_names: &Vec<&str>,
    schema: &Schema,
    source: &Source,
    namespace_str_path: &Vec<&str>,
    filter: &Arc<dyn Fn(&Node) -> bool>,
    availability: Availability,
) -> Option<Vec<usize>> {
    search_identifier_path_names_with_filter_to_top(
        identifier_path_names,
        schema,
        source,
        namespace_str_path,
        filter,
        availability
    ).map(|t| t.path().clone())
}

pub fn search_identifier_path_names_with_filter_to_top<'a>(
    identifier_path_names: &Vec<&str>,
    schema: &'a Schema,
    source: &'a Source,
    namespace_str_path: &Vec<&str>,
    filter: &Arc<dyn Fn(&Node) -> bool>,
    availability: Availability,
) -> Option<&'a Node> {
    let mut used_sources = vec![];
    let reference = search_identifier_path_names_in_source_to_top(
        identifier_path_names,
        schema,
        filter,
        source,
        &mut used_sources,
        namespace_str_path,
        availability,
    );
    if reference.is_none() {
        for builtin_source in schema.builtin_sources() {
            if let Some(reference) = search_identifier_path_names_in_source_to_top(
                &identifier_path_names,
                schema,
                filter,
                builtin_source,
                &mut used_sources,
                &vec!["std"],
                availability,
            ) {
                return Some(reference);
            }
        }
    }
    reference
}

fn search_identifier_path_names_in_source_to_top<'a>(
    identifier_path_names: &Vec<&str>,
    schema: &'a Schema,
    filter: &Arc<dyn Fn(&Node) -> bool>,
    source: &'a Source,
    used_sources: &mut Vec<usize>,
    ns_str_path: &Vec<&str>,
    availability: Availability,
) -> Option<&'a Node> {
    if used_sources.contains(&source.id) {
        return None;
    }
    used_sources.push(source.id);
    let mut ns_str_path_mut = ns_str_path.clone();
    loop {
        if ns_str_path_mut.is_empty() {
            if let Some(top) = source.find_top_by_string_path(identifier_path_names, filter, availability) {
                return Some(top);
            }
        } else {
            if let Some(ns) = source.find_child_namespace_by_string_path(&ns_str_path_mut) {
                if let Some(top) = ns.find_top_by_string_path(identifier_path_names, filter, availability) {
                    return Some(top);
                }
            }
        }
        if ns_str_path_mut.len() > 0 {
            ns_str_path_mut.pop();
        } else {
            break
        }
    }
    for import in source.imports() {
        // find with imports
        if let Some(from_source) = schema.sources().iter().find(|source| {
            import.file_path.as_str() == source.file_path.as_str()
        }).map(|s| *s) {
            if let Some(found) = search_identifier_path_names_in_source_to_top(identifier_path_names, schema, filter, from_source, used_sources, &ns_str_path, availability) {
                return Some(found)
            }
        }
    }
    None
}

fn top_to_reference_type_and_value(top: &Node) -> ExprInfo {
    ExprInfo {
        r#type: match top {
            Node::Import(_) => Type::Undetermined,
            Node::Config(c) => Type::ConfigReference(Reference::new(c.path.clone(), c.string_path.clone())),
            Node::ConfigDeclaration(_) => Type::Undetermined,
            Node::Constant(c) => return c.resolved().clone(),
            Node::Enum(e) => Type::EnumReference(Reference::new(e.path.clone(), e.string_path.clone())),
            Node::Model(m) => Type::ModelReference(Reference::new(m.path.clone(), m.string_path.clone())),
            Node::DataSet(d) => Type::DataSetReference(d.string_path.clone()),
            Node::MiddlewareDeclaration(m) => Type::MiddlewareReference(Reference::new(m.path.clone(), m.string_path.clone())),
            Node::HandlerGroupDeclaration(_) => Type::Undetermined,
            Node::InterfaceDeclaration(i) => if i.generics_declaration.is_none() {
                Type::InterfaceReference(Reference::new(i.path.clone(), i.string_path.clone()), vec![])
            } else {
                Type::Undetermined
            },
            Node::Namespace(n) => Type::NamespaceReference(n.string_path.clone()),
            Node::DecoratorDeclaration(d) => Type::DecoratorReference(Reference::new(d.path.clone(), d.string_path.clone())),
            Node::PipelineItemDeclaration(p) => Type::PipelineItemReference(Reference::new(p.path.clone(), p.string_path.clone())),
            Node::StructDeclaration(s) => if s.generics_declaration.is_none() {
                Type::StructReference(Reference::new(s.path.clone(), s.string_path.clone()), vec![])
            } else {
                Type::Undetermined
            }
            Node::UseMiddlewaresBlock(_) => Type::Undetermined,
            _ => Type::Undetermined,
        },
        value: None,
    }
}

