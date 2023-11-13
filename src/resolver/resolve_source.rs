use crate::resolver::resolve_handler_group::{resolve_handler_group_decorators, resolve_handler_group_types};
use crate::resolver::resolve_config::resolve_config;
use crate::resolver::resolve_config_declaration::resolve_config_declaration;
use crate::resolver::resolve_constant::resolve_constant;
use crate::resolver::resolve_data_set::{resolve_data_set, resolve_data_set_records};
use crate::resolver::resolve_decorator_declaration::resolve_decorator_declaration;
use crate::resolver::resolve_enum::resolve_enum;
use crate::resolver::resolve_interface::resolve_interface_declaration;
use crate::resolver::resolve_middleware::resolve_middleware;
use crate::resolver::resolve_model::{resolve_model_decorators, resolve_model_info};
use crate::resolver::resolve_namespace::{resolve_namespace_first, resolve_namespace_second, resolve_namespace_third};
use crate::resolver::resolve_pipeline_item_declaration::resolve_pipeline_item_declaration;
use crate::resolver::resolve_struct_declaration::resolve_struct_declaration;
use crate::resolver::resolve_use_middlewares_block::resolve_use_middlewares_block;
use crate::resolver::resolver_context::ResolverContext;

pub(super) fn resolve_source_first<'a>(context: &'a ResolverContext<'a>) {
    for top in context.source().children() {
        match top {
            Top::Import(_) => (), // resolved when parsing,
            Top::Constant(constant) => resolve_constant(constant, context),
            Top::Enum(r#enum) => resolve_enum(r#enum, context),
            Top::Model(model) => resolve_model_info(model, context),
            Top::Config(config) => resolve_config(config, context),
            Top::DataSet(_) => (), // do not resolve yet
            Top::Middleware(middleware) => resolve_middleware(middleware, context),
            Top::Interface(interface) => resolve_interface_declaration(interface, context),
            Top::Namespace(namespace) => resolve_namespace_first(namespace, context),
            Top::ConfigDeclaration(config_declaration) => resolve_config_declaration(config_declaration, context),
            Top::HandlerGroup(handler_group) => resolve_handler_group_types(handler_group, context),
            Top::DecoratorDeclaration(d) => resolve_decorator_declaration(d, context),
            Top::PipelineItemDeclaration(p) => resolve_pipeline_item_declaration(p, context),
            Top::StructDeclaration(s) => resolve_struct_declaration(s, context),
            Top::UseMiddlewareBlock(u) => (),
        }
    }
}

pub(super) fn resolve_source_second<'a>(context: &'a ResolverContext<'a>) {
    for top in context.source().children() {
        match top {
            Top::DataSet(data_set) => resolve_data_set(data_set, context),
            Top::Namespace(namespace) => resolve_namespace_second(namespace, context),
            Top::Model(model) => resolve_model_decorators(model, context),
            Top::HandlerGroup(handler_group) => resolve_handler_group_decorators(handler_group, context),
            Top::UseMiddlewareBlock(u) => resolve_use_middlewares_block(u, context),
            _ => ()
        }
    }
}

pub(super) fn resolve_source_third<'a>(context: &'a ResolverContext<'a>) {
    for top in context.source().children() {
        match top {
            Top::DataSet(data_set) => resolve_data_set_records(data_set, context),
            Top::Namespace(namespace) => resolve_namespace_third(namespace, context),
            _ => ()
        }
    }
}