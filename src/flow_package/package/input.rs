use crate::package::type_description::TypeDescription;
use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Input {
    #[serde(rename = "type")]
    pub input_type: TypeDescription,
}
