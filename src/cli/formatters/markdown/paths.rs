use super::operations::OperationsPrinter;
use crate::diff::paths::PathsDiff;
use crate::openapi::{Parameter, ReferenceOr};

pub struct PathsPrinter<'a> {
    pub diff: &'a PathsDiff,
}

impl<'a> PathsPrinter<'a> {
    pub fn print(&self) -> String {
        let mut result = String::new();

        result.push_str("## Paths\n\n");
        result.push_str(&self.print_added_paths());
        result.push_str(&self.print_removed_paths());
        result.push_str(&self.print_changed_paths());

        result
    }

    pub fn print_changed_paths(&self) -> String {
        let mut result = String::new();

        result.push_str("### Changed\n\n");

        for (path, path_item_diff) in &self.diff.changed {
            result.push_str(format!("##### `{}`\n", path).as_str());

            for operation_method in &path_item_diff.operations_added {
                match &operation_method.1.operation_id {
                    Some(op_id) => {
                        result.push_str(
                            format!(
                                "  - Operation {} was added for method {}\n",
                                op_id, operation_method.0
                            )
                            .as_str(),
                        );
                    }
                    None => {
                        result.push_str(
                            format!(
                                "  - An operation without an id was added for method {}\n",
                                operation_method.0
                            )
                            .as_str(),
                        );
                    }
                }
            }

            result.push_str("\n");

            for operation_method in &path_item_diff.operations_removed {
                match &operation_method.1.operation_id {
                    Some(op_id) => {
                        result.push_str(
                            format!(
                                "  - Operation {} was removed for method {}\n",
                                op_id, operation_method.0
                            )
                            .as_str(),
                        );
                    }
                    None => {
                        result.push_str(
                            format!(
                                "  - An operation without an id was removed for method {}\n",
                                operation_method.0
                            )
                            .as_str(),
                        );
                    }
                }
            }

            for (method, operation_diff) in &path_item_diff.operations_changed {
                result.push_str(format!("##### `{} {}`\n\n", method.to_uppercase(), path).as_str());

                let op_diff = OperationsPrinter {
                    operation_diff: &operation_diff,
                }
                .print();

                result.push_str(&op_diff);
            }
        }

        result.push_str("\n");

        result
    }

    pub fn print_removed_paths(&self) -> String {
        let mut result = String::new();

        result.push_str("### Removed\n\n");

        for removed in &self.diff.removed {
            result.push_str(format!("  - {}\n", removed.0).as_str());
        }

        result.push_str("\n");

        result
    }

    pub fn print_added_paths(&self) -> String {
        let mut result = String::new();

        result.push_str("### Added\n\n");

        for added in &self.diff.added {
            if let ReferenceOr::Item(path_item) = &added.1 {
                if let Some(get) = &path_item.get {
                    result.push_str(
                        format!(
                            "  - {} {} ({})\n",
                            "GET",
                            added.0,
                            get.operation_id.clone().unwrap_or("No operation id".into())
                        )
                        .as_str(),
                    );
                }

                if let Some(post) = &path_item.post {
                    result.push_str(
                        format!(
                            "  - {} {} ({})\n",
                            "POST",
                            added.0,
                            post.operation_id
                                .clone()
                                .unwrap_or("No operation id".into())
                        )
                        .as_str(),
                    );
                }

                if let Some(put) = &path_item.put {
                    result.push_str(
                        format!(
                            "  - {} {} ({})\n",
                            "PUT",
                            added.0,
                            put.operation_id.clone().unwrap_or("No operation id".into())
                        )
                        .as_str(),
                    );
                }

                if let Some(patch) = &path_item.patch {
                    result.push_str(
                        format!(
                            "  - {} {} ({})\n",
                            "PATCH",
                            added.0,
                            patch
                                .operation_id
                                .clone()
                                .unwrap_or(String::from("No operation id"))
                        )
                        .as_str(),
                    );
                }
            } else {
                result.push_str(format!("  - {}\n", added.0).as_str());
            }
        }

        result.push_str("\n");

        result
    }
}
