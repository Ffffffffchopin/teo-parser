use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};
use crate::parser::ast::action::ActionGroupDeclaration;
use crate::parser::ast::client::ASTClient;
use crate::parser::ast::config::ASTServer;
use crate::parser::ast::connector::ASTConnector;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::data_set::ASTDataSet;
use crate::parser::ast::debug_conf::ASTDebugConf;
use crate::parser::ast::generator::ASTEntity;
use crate::parser::ast::import::ASTImport;
use crate::parser::ast::interface::InterfaceDeclaration;
use crate::parser::ast::middleware::MiddlewareDeclaration;
use crate::parser::ast::model::ASTModel;
use crate::parser::ast::r#enum::ASTEnum;
use crate::parser::ast::span::Span;
use crate::parser::ast::static_files::StaticFiles;
use crate::parser::ast::test_conf::ASTTestConf;
use crate::parser::ast::top::Top;

pub(crate) struct ASTNamespace {
    pub(crate) source_id: usize,
    pub(crate) parent_ids: Vec<usize>,
    pub(crate) id: usize,
    pub(crate) span: Span,
    pub(crate) name: String,
    pub(crate) tops: BTreeMap<usize, Top>,
    pub(crate) imports: BTreeSet<usize>,
    pub(crate) constants: BTreeSet<usize>,
    pub(crate) enums: BTreeSet<usize>,
    pub(crate) models: BTreeSet<usize>,
    pub(crate) namespaces: BTreeSet<usize>,
    pub(crate) data_sets: BTreeSet<usize>,
    pub(crate) resolved: bool,
}

impl ASTNamespace {
    pub(crate) fn new(
        source_id: usize, parent_ids: Vec<usize>, id: usize, span: Span, name: String,
        tops: BTreeMap<usize, Top>, imports: BTreeSet<usize>, constants: BTreeSet<usize>, enums: BTreeSet<usize>,
        models: BTreeSet<usize>, namespaces: BTreeSet<usize>, data_sets: BTreeSet<usize>,
    ) -> Self {
        Self {
            source_id, parent_ids, id, span, name, tops, imports, constants, enums, models, namespaces, data_sets, resolved: false
        }
    }

    pub(crate) fn imports(&self) -> ASTNamespaceImportIter {
        ASTNamespaceImportIter { ns: self, index: 0 }
    }

    pub(crate) fn get_import(&self, id: usize) -> &ASTImport {
        self.tops.get(&id).unwrap().as_import().unwrap()
    }

    pub(crate) fn get_constant(&self, id: usize) -> &Constant {
        self.tops.get(&id).unwrap().as_constant().unwrap()
    }

    pub(crate) fn get_enum(&self, id: usize) -> &ASTEnum {
        self.tops.get(&id).unwrap().as_enum().unwrap()
    }

    pub(crate) fn get_model(&self, id: usize) -> &ASTModel {
        self.tops.get(&id).unwrap().as_model().unwrap()
    }

    pub(crate) fn get_namespace(&self, id: usize) -> &ASTNamespace {
        self.tops.get(&id).unwrap().as_namespace().unwrap()
    }

    pub(crate) fn get_connector(&self, id: usize) -> &ASTConnector {
        self.tops.get(&id).unwrap().as_connector().unwrap()
    }

    pub(crate) fn get_server(&self, id: usize) -> &ASTServer {
        self.tops.get(&id).unwrap().as_server_config().unwrap()
    }

    pub(crate) fn get_entity(&self, id: usize) -> &ASTEntity {
        self.tops.get(&id).unwrap().as_generator().unwrap()
    }

    pub(crate) fn get_client(&self, id: usize) -> &ASTClient {
        self.tops.get(&id).unwrap().as_client().unwrap()
    }

    pub(crate) fn get_data_set(&self, id: usize) -> &ASTDataSet {
        self.tops.get(&id).unwrap().as_data_set().unwrap()
    }

    pub(crate) fn get_debug_conf(&self, id: usize) -> &ASTDebugConf {
        self.tops.get(&id).unwrap().as_debug_conf().unwrap()
    }

    pub(crate) fn get_test_conf(&self, id: usize) -> &ASTTestConf {
        self.tops.get(&id).unwrap().as_test_conf().unwrap()
    }

    pub(crate) fn get_middleware(&self, id: usize) -> &MiddlewareDeclaration {
        self.tops.get(&id).unwrap().as_middleware().unwrap()
    }

    pub(crate) fn get_action_group(&self, id: usize) -> &ActionGroupDeclaration {
        self.tops.get(&id).unwrap().as_action_group().unwrap()
    }

    pub(crate) fn get_interface(&self, id: usize) -> &InterfaceDeclaration {
        self.tops.get(&id).unwrap().as_interface().unwrap()
    }

    pub(crate) fn get_static_files(&self, id: usize) -> &StaticFiles {
        self.tops.get(&id).unwrap().as_static_files().unwrap()
    }


    pub(crate) fn models(&self) -> Vec<&ASTModel> {
        self.models.iter().map(|m| self.get_model(*m)).collect()
    }

    pub(crate) fn enums(&self) -> Vec<&ASTEnum> {
        self.enums.iter().map(|m| self.get_enum(*m)).collect()
    }

    pub(crate) fn action_groups(&self) -> Vec<&ActionGroupDeclaration> {
        self.namespaces.iter().map(|m| self.get_action_group(*m)).collect()
    }

    pub(crate) fn namespaces(&self) -> Vec<&ASTNamespace> {
        self.namespaces.iter().map(|m| self.get_namespace(*m)).collect()
    }

    pub(crate) fn data_sets(&self) -> Vec<&ASTDataSet> {
        self.data_sets.iter().map(|m| self.get_data_set(*m)).collect()
    }

    pub(crate) fn get_model_by_name(&self, name: &str) -> Option<&ASTModel> {
        self.models().iter().find(|m| m.identifier.name.as_str() == name).map(|r| *r)
    }

    pub(crate) fn get_enum_by_name(&self, name: &str) -> Option<&ASTEnum> {
        self.enums().iter().find(|m| m.identifier.name.as_str() == name).map(|r| *r)
    }

    pub(crate) fn get_namespace_by_path(&self, path: Vec<&str>) -> Option<&ASTNamespace> {
        let mut retval = self;
        for item in path {
            if let Some(child) = retval.namespaces().iter().find(|n| n.name.as_str() == item) {
                retval = child
            } else {
                return None
            }
        }
        Some(retval)
    }

    pub(crate) fn get_model_by_path(&self, path: Vec<&str>) -> Option<&ASTModel> {
        if path.len() == 1 {
            self.get_model_by_name(path.get(0).unwrap())
        } else {
            let mut path_for_ns = path.clone();
            path_for_ns.remove(path_for_ns.len() - 1);
            let child_ns = self.get_namespace_by_path(path_for_ns.clone());
            return if let Some(child_ns) = child_ns {
                child_ns.get_model_by_name(path_for_ns.last().unwrap())
            } else {
                None
            }
        }
    }

    pub(crate) fn get_enum_by_path(&self, path: Vec<&str>) -> Option<&ASTEnum> {
        if path.len() == 1 {
            self.get_enum_by_name(path.get(0).unwrap())
        } else {
            let mut path_for_ns = path.clone();
            path_for_ns.remove(path_for_ns.len() - 1);
            let child_ns = self.get_namespace_by_path(path_for_ns.clone());
            return if let Some(child_ns) = child_ns {
                child_ns.get_enum_by_name(path_for_ns.last().unwrap())
            } else {
                None
            }
        }
    }
}

impl Debug for ASTNamespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Namespace")
    }
}

pub(crate) struct ASTNamespaceImportIter<'a> {
    ns: &'a ASTNamespace,
    index: usize,
}

impl<'a> Iterator for ASTNamespaceImportIter<'a> {
    type Item = &'a ASTImport;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ns.imports.iter().skip(self.index).next() {
            Some(index) => {
                let top = self.ns.tops.get(index).unwrap();
                self.index += 1;
                Some(top.as_import().unwrap())
            }
            None => None,
        }

    }
}