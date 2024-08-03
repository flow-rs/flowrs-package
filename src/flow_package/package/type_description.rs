use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TypeDescription {
    Type {
        name: String,
        type_parameters: Option<Vec<Box<TypeDescription>>>,
    },

    Generic {
        name: String,
        type_parameters: Option<Vec<Box<TypeDescription>>>,
    },
}
