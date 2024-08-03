use crate::package::type_description::TypeDescription;
use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Output {
    #[serde(rename = "type")]
    pub output_type: TypeDescription,
}
