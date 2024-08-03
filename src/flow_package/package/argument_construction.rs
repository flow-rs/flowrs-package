use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ArgumentConstruction {
    Constructor(String),
    ExistingObject(),
}
