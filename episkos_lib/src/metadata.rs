//! # Metadata used within the project
//!
//! This module contains the metadata upon which the project is based.
//!
//! ## Important interfaces
//! blablabla
//! ### Builder
//! A metadata object can be created using the MetadataBuilder.
//!
//! #### Example
//! ```
//! use episkos_lib::metadata::Metadata;
//! use std::path::Path;
//!
//! // Creating a minimal metadata object
//! let metadata = Metadata::builder()
//!     .title("Example Project")
//!     .directory(".")
//!     .build()
//!     .unwrap();
//!
//! ```
//! ### Validation
//! ### Properties
//! #### Simple
//! #### Advanced
use std::{
    io,
    path::{Path, PathBuf},
};

use builder::MetadataBuilder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use uuid::Uuid;

pub mod build_system;
mod builder;
pub mod category;
pub mod ide;
pub mod language;
pub mod property;

pub use build_system::BuildSystem;
pub use category::Category;
pub use ide::Ide;
pub use language::Language;

/// Metadata structure containing information about a project.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    id: Uuid,
    #[serde(skip)]
    directory: PathBuf,
    title: String,
    categories: Vec<Category>,
    languages: Vec<Language>,
    preffered_ide: Option<Ide>,
    build_systems: Vec<BuildSystem>,
    description: Option<String>,
    repository_url: Option<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl Metadata {
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder::new()
    }

    pub fn update(self) -> MetadataBuilder {
        MetadataBuilder::from_metadata(self)
    }

    pub fn directory(&self) -> &Path {
        &self.directory
    }

    pub fn update_directory(&mut self, path: PathBuf) {
        self.directory = path
    }

    pub fn get_hash(&self) -> Result<[u8; 32], Error> {
        let string = toml::to_string(self)?;

        let mut hasher = Sha256::new();
        hasher.update(string);
        Ok(hasher.finalize().into())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to build Metadata")]
    FailedToBuild(#[from] builder::Error),

    #[error("io error")]
    Io(#[from] io::Error),

    #[error("serialization error")]
    Serialization(#[from] toml::ser::Error),
}

#[cfg(test)]
mod tests {
    

    use super::*;

    #[test]
    fn test_metadata_checksum_is_consistent() {
        let metadata = get_simple_metadata();
        let checksum1 = metadata.get_hash().unwrap();
        for i in 0..100 {
            let checksum2 = metadata.get_hash().unwrap();
            assert_eq!(checksum1, checksum2)
        }
    }

    #[test]
    fn test_metadata_checksum_is_changing() {
        let metadata = get_simple_metadata();
        let checksum1 = metadata.get_hash().unwrap();

        let metadata = metadata.update().build().unwrap();
        let checksum2 = metadata.get_hash().unwrap();
        assert_ne!(checksum1, checksum2);

        let metadata = metadata.update().title("Fun").build().unwrap();
        let checksum3 = metadata.get_hash().unwrap();
        assert_ne!(checksum1, checksum3);
        assert_ne!(checksum2, checksum3);
    }

    fn get_simple_metadata() -> Metadata {
        Metadata::builder()
            .title("Hello")
            .directory("/")
            .build()
            .unwrap()
    }
}
