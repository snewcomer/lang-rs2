use crate::val::Val;
use crate::stmt::Stmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct Env<'parent> {
    named: HashMap<String, NamedInfo>,
    parent: Option<&'parent Self>,
}

#[derive(Clone, Debug, PartialEq)]
enum NamedInfo {
    Binding(Val),
    Func { params: Vec<String>, body: Stmt },
}

impl<'parent> Env<'parent> {
    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            named: HashMap::new(),
            parent: Some(self),
        }
    }

    pub(crate) fn store_binding(&mut self, name: String, val: Val) {
        self.named.insert(name, NamedInfo::Binding(val));
    }

    pub(crate) fn store_func(&mut self, name: String, params: Vec<String>, body: Stmt) {
        self.named.insert(name, NamedInfo::Func { params, body });
    }

    pub(crate) fn get_binding(&self, name: &str) -> Result<Val, String> {
        self.get_named_info(name)
            .and_then(|named_info| match named_info {
                NamedInfo::Binding(val) => Some(val),
                _ => None,
            })
            .ok_or_else(|| format!("binding with name '{}' does not exist", name))
    }

    pub(crate) fn get_func(&self, name: &str) -> Result<(Vec<String>, Stmt), String> {
        self.get_named_info(name)
            .and_then(|named_info| match named_info {
                NamedInfo::Func { params, body } => Some((params, body)),
                _ => None,
            })
            .ok_or_else(|| format!("function with name ‘{}’ does not exist", name))
    }

    fn get_named_info(&self, name: &str) -> Option<NamedInfo> {
        self.named
            .get(name)
            .cloned()
            .or_else(|| self.parent.and_then(|parent| parent.get_named_info(name)))
    }
}
