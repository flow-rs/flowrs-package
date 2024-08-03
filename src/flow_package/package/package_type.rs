use crate::package::constructor::Constructor;
use crate::package::input::Input;
use crate::package::output::Output;
use crate::package::type_parameter::TypeParameter;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Type {
    pub inputs: Option<HashMap<String, Input>>,
    pub outputs: Option<HashMap<String, Output>>,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub constructors: HashMap<String, Constructor>,
}

impl Type {
    pub fn new_with_constructor(constructor_name: &str, constructor: Constructor) -> Self {
        let mut t = Self {
            inputs: Option::None,
            outputs: Option::None,
            type_parameters: Option::None,
            constructors: HashMap::new(),
        };
        t.constructors.insert(constructor_name.into(), constructor);
        t
    }

    pub fn new_primitive_type() -> Self {
        let mut t = Self {
            inputs: Option::None,
            outputs: Option::None,
            type_parameters: Option::None,
            constructors: HashMap::new(),
        };
        t.constructors
            .insert("Default".into(), Constructor::FromDefault);
        t.constructors.insert("Json".into(), Constructor::FromJson);

        t
    }

    pub fn new_simple() -> Self {
        Self {
            inputs: Option::None,
            outputs: Option::None,
            type_parameters: Option::None,
            constructors: HashMap::new(),
        }
    }
}
