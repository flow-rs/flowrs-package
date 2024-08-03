use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ArgumentPassing {
    Reference,
    MutableReference,
    Move,
    Clone,
}
