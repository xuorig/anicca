use super::operations::OperationDiff;
use super::DiffError;
use openapiv3::{Operation, PathItem, ReferenceOr};
use serde::Serialize;
use std::collections::HashMap;

pub(crate) type PathItemPair = (String, ReferenceOr<PathItem>);

type OperationMethod = (String, Operation);

#[derive(Debug, Default, Serialize)]
pub(crate) struct PathItemDiff {
    operations_added: Vec<OperationMethod>,
    operations_removed: Vec<OperationMethod>,
    operations_changed: HashMap<String, OperationDiff>,
}

impl PathItemDiff {
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
        let mut operations_changed: HashMap<String, OperationDiff> = HashMap::default();

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
                Some(head_op) => {
                    let diff = OperationDiff::from_operations(op, head_op);

                    if diff.has_changes() {
                        operations_changed.insert(String::from("get"), diff);
                    }
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
            operations_changed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations_removed() {
        let mut path_item = PathItem::default();
        let mut operation = Operation::default();
        operation.operation_id = Some("cats/get".into());
        path_item.get = Some(operation);

        let base = ReferenceOr::Item(path_item);

        let head = ReferenceOr::Item(PathItem::default());

        let diff = PathItemDiff::from_path_items(&base, &head).expect("Failed to diff paths");

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

        let diff = PathItemDiff::from_path_items(&base, &head).expect("Failed to diff paths");

        assert_eq!(diff.operations_removed.len(), 0);
        assert_eq!(diff.operations_added.len(), 1);
        let added_op = diff.operations_added.first().unwrap();

        assert_eq!(added_op.0, "get");
        assert_eq!(added_op.1.operation_id, Some("cats/get".into()));
    }
}
