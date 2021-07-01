use crate::operations::OperationDiff;
use crate::DiffError;
use openapiv3::{PathItem, Paths, ReferenceOr};
use std::collections::HashMap;

type PathItemPair = (String, ReferenceOr<PathItem>);

#[derive(Debug, Default)]
pub(crate) struct PathsDiff {
    paths_added: Vec<PathItemPair>,
    paths_removed: Vec<PathItemPair>,
    paths_changed: HashMap<String, PathDiff>,
}

#[derive(Debug, Default)]
pub(crate) struct PathDiff {}

impl PathDiff {
    pub fn has_change(&self) -> bool {
        false
    }
}

/// Diffs two sets of OpenAPI paths
pub(crate) fn diff_paths(base: &Paths, head: &Paths) -> Result<PathsDiff, DiffError> {
    let mut paths_added = vec![];
    let mut paths_removed = vec![];
    let mut paths_changed: HashMap<String, PathDiff> = HashMap::new();

    for (path, path_item) in base {
        match head.get(path) {
            Some(head_path_item) => {
                let path_item_diff = diff_path(path_item, head_path_item)?;

                if path_item_diff.has_change() {
                    paths_changed.insert(path.clone(), path_item_diff);
                }
            }
            None => paths_removed.push((path.clone(), path_item.clone())),
        }
    }

    for (path, path_item) in head {
        match base.get(path) {
            Some(_) => {}
            None => paths_added.push((path.clone(), path_item.clone())),
        }
    }

    Ok(PathsDiff {
        paths_added,
        paths_removed,
        paths_changed,
    })
}

/// Diffs two sets of OpenAPI paths
pub(crate) fn diff_path(
    base: &ReferenceOr<PathItem>,
    head: &ReferenceOr<PathItem>,
) -> Result<PathDiff, DiffError> {
    Ok(PathDiff::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_added() {
        let base = Paths::default();
        let mut head = Paths::default();
        head.insert("/cats".into(), ReferenceOr::Item(PathItem::default()));

        let diff = diff_paths(&base, &head).expect("Failed to diff paths");

        assert_eq!(diff.paths_added.len(), 1);
        assert_eq!(diff.paths_added.first().unwrap().0, "/cats");
    }

    #[test]
    fn path_removed() {
        let mut base = Paths::default();
        base.insert("/cats".into(), ReferenceOr::Item(PathItem::default()));
        let head = Paths::default();

        let diff = diff_paths(&base, &head).expect("Failed to diff paths");

        assert_eq!(diff.paths_added.len(), 0);
        assert_eq!(diff.paths_removed.len(), 1);
        assert_eq!(diff.paths_removed.first().unwrap().0, "/cats");
    }
}
