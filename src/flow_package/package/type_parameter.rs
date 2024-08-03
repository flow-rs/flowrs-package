use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TypeParameter {
    pub name: String,

    // We only support trait constraints as is T::Default or T::Clone
    #[serde(rename = "where")]
    pub constraints: Vec<String>,
}
