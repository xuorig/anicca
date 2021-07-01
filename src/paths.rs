use crate::operations::OperationDiff;
use crate::DiffError;
use openapiv3::{Operation, PathItem, Paths, ReferenceOr};
use std::collections::HashMap;

type PathItemPair = (String, ReferenceOr<PathItem>);

#[derive(Debug, Default)]
pub(crate) struct PathsDiff {
    paths_added: Vec<PathItemPair>,
    paths_removed: Vec<PathItemPair>,
    paths_changed: HashMap<String, PathDiff>,
}

impl PathsDiff {
    /// Diffs two sets of OpenAPI paths
    pub(crate) fn from_paths(base: &Paths, head: &Paths) -> Result<Self, DiffError> {
        let mut paths_added = vec![];
        let mut paths_removed = vec![];
        let mut paths_changed: HashMap<String, PathDiff> = HashMap::new();

        for (path, path_item) in base {
            match head.get(path) {
                Some(head_path_item) => {
                    let path_item_diff = PathDiff::from_path_items(path_item, head_path_item)?;

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

type OperationMethod = (String, Operation);

#[derive(Debug, Default)]
pub(crate) struct PathDiff {
    operations_added: Vec<OperationMethod>,
    operations_removed: Vec<OperationMethod>,
}

impl PathDiff {
    pub fn has_change(&self) -> bool {
        !self.operations_removed.is_empty() || !self.operations_added.is_empty()
    }

    /// Diffs two sets of OpenAPI paths
    pub fn from_path_items(
        base: &ReferenceOr<PathItem>,
        head: &ReferenceOr<PathItem>,
    ) -> Result<Self, DiffError> {
        let mut operations_added = vec![];
        let mut operations_removed = vec![];

        let base_path_item = match &base {
            ReferenceOr::Item(pi) => pi,
            ReferenceOr::Reference { reference } => {
                return Err(DiffError::UnsupportedFeature(format!(
                    "Cannot diff reference {} path references is not implemented yet",
                    reference
                )));
            }
        };

        let head_path_item = match &head {
            ReferenceOr::Item(pi) => pi,
            ReferenceOr::Reference { reference } => {
                return Err(DiffError::UnsupportedFeature(format!(
                    "Cannot diff reference {} path references is not implemented yet",
                    reference
                )));
            }
        };

        match &base_path_item.get {
            Some(op) => match &head_path_item.get {
                Some(_head_op) => {
                    // Diff the get method
                }
                None => {
                    // Removed
                    operations_removed.push((String::from("get"), op.clone()));
                }
            },
            None => match &head_path_item.get {
                Some(head_op) => {
                    // Added
                    operations_added.push((String::from("get"), head_op.clone()));
                }
                None => {}
            },
        }

        Ok(Self {
            operations_added,
            operations_removed,
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct PathItemDiff {}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn operations_removed() {
        let mut path_item = PathItem::default();
        let mut operation = Operation::default();
        operation.operation_id = Some("cats/get".into());
        path_item.get = Some(operation);

        let base = ReferenceOr::Item(path_item);

        let head = ReferenceOr::Item(PathItem::default());

        let diff = PathDiff::from_path_items(&base, &head).expect("Failed to diff paths");

        assert_eq!(diff.operations_removed.len(), 1);
        let removed_op = diff.operations_removed.first().unwrap();

        assert_eq!(removed_op.0, "get");
        assert_eq!(removed_op.1.operation_id, Some("cats/get".into()));
    }

    #[test]
    fn operations_added() {
        let mut path_item = PathItem::default();
        let mut operation = Operation::default();
        operation.operation_id = Some("cats/get".into());
        path_item.get = Some(operation);

        let base = ReferenceOr::Item(PathItem::default());
        let head = ReferenceOr::Item(path_item);

        let diff = PathDiff::from_path_items(&base, &head).expect("Failed to diff paths");

        assert_eq!(diff.operations_removed.len(), 0);
        assert_eq!(diff.operations_added.len(), 1);
        let added_op = diff.operations_added.first().unwrap();

        assert_eq!(added_op.0, "get");
        assert_eq!(added_op.1.operation_id, Some("cats/get".into()));
    }
}
