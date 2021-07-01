use crate::operations::OperationDiff;
use crate::DiffError;
use openapiv3::Paths;
use std::collections::HashMap;

type PathItemDiff = (String, OperationDiff);

#[derive(Debug, Default)]
pub(crate) struct PathsDiff {
    paths_added: Vec<openapiv3::PathItem>,
    paths_removed: Vec<openapiv3::PathItem>,
    paths_changed: HashMap<String, PathDiff>,
}

#[derive(Debug, Default)]
pub(crate) struct PathDiff {
    path_items_added: Vec<String>,
    path_items_removed: Vec<String>,
    path_items_changed: Vec<PathItemDiff>,
}

/// Diffs two sets of OpenAPI paths
pub(crate) fn diff_paths(base: Paths, head: Paths) -> Result<PathsDiff, DiffError> {
    Ok(PathsDiff::default())
}
