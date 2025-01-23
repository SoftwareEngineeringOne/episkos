use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSystem {
    pub name: String,
    pub version: String,
}
