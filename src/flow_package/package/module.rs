use crate::package::package_type::Type;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Module {
    pub types: HashMap<String, Type>,
    pub modules: HashMap<String, Module>,
}
