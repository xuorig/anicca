pub(crate) mod operations;
pub(crate) mod paths;

use openapiv3::OpenAPI;
use paths::{diff_paths, PathsDiff};
use serde_json;
use thiserror::Error;

/// DiffError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum DiffError {
    /// Represents an invalid OpenAPI document according to the specification.
    #[error("Invalid OpenAPI document")]
    InvalidOpenAPI,

    /// Represents all cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Represents all cases of `serde_json::Error`.
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

/// OpenAPIVersionChange represents a change in the OpenAPI specification version
/// between documents.
#[derive(Debug)]
pub struct OpenAPIVersionDiff {
    from: String,
    to: String,
}

impl OpenAPIVersionDiff {
    pub fn message(&self) -> String {
        format!(
            "OpenAPI specification version changed from {} to {}.",
            self.from, self.to
        )
    }
}

#[derive(Debug)]
pub struct Diff {
    version_diff: Option<OpenAPIVersionDiff>,
    paths_diff: PathsDiff,
}

pub fn diff_json(base: &str, head: &str) -> Result<Diff, DiffError> {
    let base_contents = std::fs::read_to_string(base)?;
    let head_contents = std::fs::read_to_string(head)?;
    let base_openapi: OpenAPI = serde_json::from_str(&base_contents)?;
    let head_openapi: OpenAPI = serde_json::from_str(&head_contents)?;
    diff(base_openapi, head_openapi)
}

pub fn diff(base: OpenAPI, head: OpenAPI) -> Result<Diff, DiffError> {
    let version_diff = diff_versions(base.openapi, head.openapi);
    let paths_diff = diff_paths(&base.paths, &head.paths)?;
    Ok(Diff {
        version_diff,
        paths_diff,
    })
}

fn diff_versions(base: String, head: String) -> Option<OpenAPIVersionDiff> {
    if base != head {
        Some(OpenAPIVersionDiff {
            from: base,
            to: head,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_json_files() {
        let diff = diff_json("fixtures/pet-store.json", "fixtures/pet-store-changed.json")
            .expect("Failed to diff JSON");

        let version_change = diff.version_diff.unwrap();

        assert_eq!(
            "OpenAPI specification version changed from 3.0.0 to 3.1.0.",
            version_change.message()
        );
    }

    #[test]
    fn openapi_version_change() {
        let mut base = OpenAPI::default();
        base.openapi = String::from("3.0.0");
        let mut head = OpenAPI::default();
        head.openapi = String::from("4.0.0");

        let result = diff(base, head);
        let diff = result.expect("Failed to diff");

        let version_change = diff.version_diff.unwrap();

        assert_eq!(
            "OpenAPI specification version changed from 3.0.0 to 4.0.0.",
            version_change.message()
        );
    }
}
