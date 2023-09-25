use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use maplit::hashmap;
use crate::core::field::field::Field;
use crate::core::model::model::Model;
use crate::core::item::Item;
use crate::core::pipeline::Pipeline;
use crate::core::property::Property;
use crate::core::relation::Relation;
use crate::ast::argument::Argument;
use crate::ast::entity::Entity;
use crate::std::callables::date_constructor::date_constructor;
use crate::std::callables::datetime_constructor::datetime_constructor;
use crate::std::callables::float_constructor::float_constructor;
use crate::std::callables::int_constructor::int_constructor;
#[cfg(feature = "data-source-mongodb")]
use crate::std::callables::object_id_constructor::object_id_constructor;
use crate::std::callables::string_constructor::string_constructor;
use crate::std::constants::EnvObject;
use crate::prelude::Value;

pub(crate) type Callable = fn(args: &Vec<Argument>) -> Value;

pub(crate) type FieldDecorator = fn(args: &'static Vec<Argument>, field: &mut Field);

pub(crate) type RelationDecorator = fn(args: &'static Vec<Argument>, relation: &mut Relation);

pub(crate) type PropertyDecorator = fn(args: &'static Vec<Argument>, property: &mut Property);

pub(crate) type ModelDecorator = fn(args: &'static Vec<Argument>, model: &mut Model);

pub(crate) type ASTPipelineInstaller = fn(args: &Vec<Argument>) -> Arc<dyn Item>;

#[derive(Clone)]
pub(crate) struct ASTPipelineItem {
    pub(crate) installer: Option<ASTPipelineInstaller>,
    pub(crate) args: Vec<Argument>,
}

impl ASTPipelineItem {
    pub(crate) fn args(&self) -> &Vec<Argument> {
        &self.args
    }
}

#[derive(Clone)]
pub(crate) struct ASTResolvedPipeline {
    pub(crate) items: Vec<ASTPipelineItem>
}

impl ASTResolvedPipeline {

    pub(crate) fn to_value_pipeline(&self) -> Pipeline {
        let mut modifiers = vec![];
        for item in self.items.iter() {
            if let Some(installer) = item.installer {
                modifiers.push((installer)(item.args()));
            }
        }
        Pipeline { items: modifiers }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Container {
    pub(crate) objects: HashMap<String, Entity>
}

impl Container {
    pub(crate) fn std_global_constants() -> Self {
        Self {
            objects: hashmap!{
                "ENV".to_owned() => Entity::Accessible(Accessible::Env(EnvObject {})),
                #[cfg(feature = "data-source-mongodb")]
                "ObjectId".to_owned() => Entity::Accessible(Accessible::Callable(object_id_constructor)),
                "Int".to_owned() => Entity::Accessible(Accessible::Callable(int_constructor)),
                "Float".to_owned() => Entity::Accessible(Accessible::Callable(float_constructor)),
                "Date".to_owned() => Entity::Accessible(Accessible::Callable(date_constructor)),
                "DateTime".to_owned() => Entity::Accessible(Accessible::Callable(datetime_constructor)),
                "String".to_owned() => Entity::Accessible(Accessible::Callable(string_constructor)),
            }
        }
    }

    pub(crate) fn access_property(&self, name: &str) -> &Entity {
        self.objects.get(name).unwrap()
    }
}

#[derive(Clone)]
pub(crate) enum Accessible {
    FieldDecorator(FieldDecorator),
    RelationDecorator(RelationDecorator),
    PropertyDecorator(PropertyDecorator),
    ModelDecorator(ModelDecorator),
    Container(Container),
    Env(EnvObject),
    Callable(Callable),
}

impl Debug for Accessible {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Accessible")
    }
}

impl Accessible {

    pub(crate) fn as_container(&self) -> Option<&Container> {
        match self {
            Accessible::Container(c) => Some(c),
            _ => None,
        }
    }

    pub(crate) fn is_container(&self) -> bool {
        self.as_container().is_some()
    }

    pub(crate) fn as_env(&self) -> Option<&EnvObject> {
        match self {
            Accessible::Env(e) => Some(e),
            _ => None,
        }
    }

    pub(crate) fn is_env(&self) -> bool {
        self.as_env().is_some()
    }

    pub(crate) fn as_field_decorator(&self) -> Option<&FieldDecorator> {
        match self {
            Accessible::FieldDecorator(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn as_relation_decorator(&self) -> Option<&RelationDecorator> {
        match self {
            Accessible::RelationDecorator(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn as_property_decorator(&self) -> Option<&PropertyDecorator> {
        match self {
            Accessible::PropertyDecorator(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn as_model_decorator(&self) -> Option<&ModelDecorator> {
        match self {
            Accessible::ModelDecorator(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn access_property(&self, name: &str) -> &Entity {
        match self.as_container() {
            Some(c) => c.access_property(name),
            None => panic!("Cannot access property '{}'", name),
        }
    }
}
