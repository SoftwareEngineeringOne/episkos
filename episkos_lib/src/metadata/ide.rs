use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ide {
    pub name: String,
    pub version: String,
}
