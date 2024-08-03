use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ObjectDescription {
    pub type_name: String,
    pub type_parameter_part: String,
    //pub type_parameters: HashMap<String, String>,
    pub name: String,
    pub is_mutable: bool,
}
