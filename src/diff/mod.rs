pub(crate) mod common;
pub(crate) mod content;
pub(crate) mod extensions;
pub(crate) mod info;
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
pub(crate) mod servers;

use crate::openapi::OpenAPI;
use common::StringDiff;
use info::InfoDiff;
use paths::PathsDiff;
use serde::Serialize;
use servers::ServersDiff;
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
    SerdeError(#[from] serde_yaml::Error),
}

#[derive(Debug, Serialize, Default)]
pub struct Diff {
    pub version: Option<StringDiff>,
    pub servers: Option<ServersDiff>,
    pub paths: Option<PathsDiff>,
    pub info: Option<InfoDiff>,
}

pub fn diff_files(base: PathBuf, head: PathBuf) -> Result<Diff, DiffError> {
    let base_contents = std::fs::read_to_string(base)?;
    let head_contents = std::fs::read_to_string(head)?;
    let base_openapi: OpenAPI = serde_yaml::from_str(&base_contents)?;
    let head_openapi: OpenAPI = serde_yaml::from_str(&head_contents)?;
    diff(base_openapi, head_openapi)
}

pub fn diff(base: OpenAPI, head: OpenAPI) -> Result<Diff, DiffError> {
    let mut diff = Diff::default();

    diff.version = StringDiff::from_strings(base.openapi, head.openapi);

    let paths_diff = PathsDiff::from_paths(&base.paths, &head.paths)?;
    if paths_diff.has_changes() {
        diff.paths = Some(paths_diff);
    }

    let info_diff = InfoDiff::from_info(&base.info, &head.info);
    if info_diff.has_changes() {
        diff.info = Some(info_diff);
    }

    let servers_diff = ServersDiff::from_servers(&base.servers, &head.servers);
    if servers_diff.has_changes() {
        diff.servers = Some(servers_diff);
    }

    Ok(diff)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_json_files() {
        let diff = diff_files(
            PathBuf::from("fixtures/pet-store.json"),
            PathBuf::from("fixtures/pet-store-changed.json"),
        )
        .expect("Failed to diff JSON");

        let version_change = diff.version.unwrap();

        assert_eq!("3.0.0", version_change.from);
        assert_eq!("3.1.0", version_change.to);
    }

    #[test]
    fn from_yaml_files() {
        let diff = diff_files(
            PathBuf::from("fixtures/pet-store.yaml"),
            PathBuf::from("fixtures/pet-store-changed.yaml"),
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
