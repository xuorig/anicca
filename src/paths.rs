use crate::path_items::{PathItemDiff, PathItemPair};
use crate::DiffError;
use openapiv3::Paths;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub(crate) struct PathsDiff {
    paths_added: Vec<PathItemPair>,
    paths_removed: Vec<PathItemPair>,
    paths_changed: HashMap<String, PathItemDiff>,
}

impl PathsDiff {
    /// Diffs two sets of OpenAPI paths
    pub(crate) fn from_paths(base: &Paths, head: &Paths) -> Result<Self, DiffError> {
        let mut paths_added = vec![];
        let mut paths_removed = vec![];
        let mut paths_changed: HashMap<String, PathItemDiff> = HashMap::new();

        for (path, path_item) in base {
            match head.get(path) {
                Some(head_path_item) => {
                    let path_item_diff = PathItemDiff::from_path_items(path_item, head_path_item)?;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use openapiv3::{PathItem, ReferenceOr};

    #[test]
    fn path_added() {
        let base = Paths::default();
        let mut head = Paths::default();
        head.insert("/cats".into(), ReferenceOr::Item(PathItem::default()));

        let diff = PathsDiff::from_paths(&base, &head).expect("Failed to diff paths");

        assert_eq!(diff.paths_added.len(), 1);
        assert_eq!(diff.paths_added.first().unwrap().0, "/cats");
    }

    #[test]
    fn path_removed() {
        let mut base = Paths::default();
        base.insert("/cats".into(), ReferenceOr::Item(PathItem::default()));
        let head = Paths::default();

        let diff = PathsDiff::from_paths(&base, &head).expect("Failed to diff paths");

        assert_eq!(diff.paths_added.len(), 0);
        assert_eq!(diff.paths_removed.len(), 1);
        assert_eq!(diff.paths_removed.first().unwrap().0, "/cats");
    }
}
