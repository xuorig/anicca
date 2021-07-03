pub(crate) mod common;
pub(crate) mod content;
pub(crate) mod media_type;
pub(crate) mod operations;
pub(crate) mod parameter;
pub(crate) mod parameters;
pub(crate) mod path_items;
pub(crate) mod paths;
pub(crate) mod request_body;
pub(crate) mod response;
pub(crate) mod responses;
pub(crate) mod schema;

use common::StringDiff;
use openapiv3::OpenAPI;
use paths::PathsDiff;
use serde::Serialize;
use std::path::PathBuf;
use thiserror::Error;

/// DiffError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum DiffError {
    /// Represents an unsupported feature by this library.
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    /// Represents all cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Represents all cases of `serde_json::Error`.
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

#[derive(Debug, Serialize)]
pub struct Diff {
    pub version: Option<StringDiff>,
    pub paths: PathsDiff,
}

pub fn diff_json(base: PathBuf, head: PathBuf) -> Result<Diff, DiffError> {
    let base_contents = std::fs::read_to_string(base)?;
    let head_contents = std::fs::read_to_string(head)?;
    let base_openapi: OpenAPI = serde_json::from_str(&base_contents)?;
    let head_openapi: OpenAPI = serde_json::from_str(&head_contents)?;
    diff(base_openapi, head_openapi)
}

pub fn diff(base: OpenAPI, head: OpenAPI) -> Result<Diff, DiffError> {
    let version_diff = StringDiff::from_strings(base.openapi, head.openapi);
    let paths_diff = PathsDiff::from_paths(&base.paths, &head.paths)?;
    Ok(Diff {
        version: version_diff,
        paths: paths_diff,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_json_files() {
        let diff = diff_json(
            PathBuf::from("fixtures/pet-store.json"),
            PathBuf::from("fixtures/pet-store-changed.json"),
        )
        .expect("Failed to diff JSON");

        let version_change = diff.version.unwrap();

        assert_eq!("3.0.0", version_change.from);
        assert_eq!("3.1.0", version_change.to);
    }

    #[test]
    fn openapi_version_change() {
        let mut base = OpenAPI::default();
        base.openapi = String::from("3.0.0");
        let mut head = OpenAPI::default();
        head.openapi = String::from("4.0.0");

        let result = diff(base, head);
        let diff = result.expect("Failed to diff");

        let version_change = diff.version.unwrap();

        assert_eq!("3.0.0", version_change.from);
        assert_eq!("4.0.0", version_change.to);
    }
}
