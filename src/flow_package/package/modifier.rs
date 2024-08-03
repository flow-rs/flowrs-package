use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Modifier {
    is_mutable: bool,
    is_reference: bool,
}
