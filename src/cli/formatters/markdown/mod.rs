use crate::diff::Diff;
use openapiv3::{Parameter, ReferenceOr};

pub struct Printer {}

impl Printer {
    pub fn print(diff: &Diff) -> String {
        let mut result = String::new();
        result.push_str("# OpenAPI diff\n\n");

        result.push_str("## General\n\n");

        if let Some(version_change) = &diff.version {
            result.push_str(
                format!(
                    "  - OpenAPI spec version changed from `{}` to `{}`.\n",
                    version_change.from, version_change.to
                )
                .as_str(),
            );
        }

        result.push_str("\n");

        result.push_str("## Paths\n\n");

        result.push_str("### Added\n\n");

        for added in &diff.paths.added {
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

        result.push_str("### Removed\n\n");

        for removed in &diff.paths.removed {
            result.push_str(format!("  - {}\n", removed.0).as_str());
        }

        result.push_str("\n");

        result.push_str("### Changed\n\n");

        for (path, path_item_diff) in &diff.paths.changed {
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

                if !operation_diff.tags.added.is_empty() {
                    result.push_str(
                        format!(
                            "  - Tags were added: `{}`\n",
                            operation_diff.tags.added.join(",")
                        )
                        .as_str(),
                    );
                }

                if !operation_diff.tags.removed.is_empty() {
                    result.push_str(
                        format!(
                            "  - Tags were removed: `{}`\n",
                            operation_diff.tags.removed.join(",")
                        )
                        .as_str(),
                    );
                }

                if let Some(summary_diff) = &operation_diff.summary {
                    if let Some(from) = &summary_diff.from {
                        if let Some(to) = &summary_diff.to {
                            result.push_str(
                                format!(
                                    "  - Operation summary changed from `{}` to `{}`.\n",
                                    from, to
                                )
                                .as_str(),
                            );
                        } else {
                            result.push_str("  - Operation summary was removed.\n");
                        }
                    } else {
                        if let Some(to) = &summary_diff.to {
                            result.push_str(
                                format!("  - Operation summary added: `{}`\n", to).as_str(),
                            );
                        }
                    }
                }

                if let Some(diff) = &operation_diff.description {
                    if let Some(from) = &diff.from {
                        if let Some(to) = &diff.to {
                            result.push_str("  - Operation description was changed.\n");
                        } else {
                            result.push_str("  - Operation description was removed.\n");
                        }
                    } else {
                        if let Some(to) = &diff.to {
                            result.push_str(
                                format!("  - Operation description added: `{}`\n", to).as_str(),
                            );
                        }
                    }
                }

                if let Some(diff) = &operation_diff.operation_id {
                    if let Some(from) = &diff.from {
                        if let Some(to) = &diff.to {
                            result.push_str(
                                format!("  - Operation id changed from `{}` to `{}`\n", from, to)
                                    .as_str(),
                            );
                        } else {
                            result.push_str("  - Operation id was removed.\n");
                        }
                    } else {
                        if let Some(to) = &diff.to {
                            result.push_str(format!("  - Operation id added: `{}`\n", to).as_str());
                        }
                    }
                }

                for param in &operation_diff.parameters.added {
                    match param {
                        ReferenceOr::Reference { reference } => {
                            result.push_str(
                                format!("  - Referenced parameter `{}` was added.\n", reference)
                                    .as_str(),
                            );
                        }
                        ReferenceOr::Item(param) => match param {
                            Parameter::Query {
                                parameter_data,
                                allow_reserved: _,
                                style: _,
                                allow_empty_value: _,
                            } => {
                                result.push_str(
                                    format!(
                                        "  - Query parameter `{}` was added.\n",
                                        parameter_data.name
                                    )
                                    .as_str(),
                                );
                            }
                            Parameter::Header {
                                parameter_data,
                                style: _,
                            } => {
                                result.push_str(
                                    format!("  - Header `{}` was added.\n", parameter_data.name)
                                        .as_str(),
                                );
                            }
                            Parameter::Path {
                                parameter_data,
                                style: _,
                            } => {
                                result.push_str(
                                    format!(
                                        "  - Path parameter `{}` was added.\n",
                                        parameter_data.name
                                    )
                                    .as_str(),
                                );
                            }
                            Parameter::Cookie {
                                parameter_data,
                                style: _,
                            } => {
                                result.push_str(
                                    format!("  - Cookie `{}` was added.\n", parameter_data.name)
                                        .as_str(),
                                );
                            }
                        },
                    }
                }

                for param in &operation_diff.parameters.removed {
                    match param {
                        ReferenceOr::Reference { reference } => {
                            result.push_str(
                                format!("  - Referenced parameter `{}` was removed.\n", reference)
                                    .as_str(),
                            );
                        }
                        ReferenceOr::Item(param) => match param {
                            Parameter::Query {
                                parameter_data,
                                allow_reserved: _,
                                style: _,
                                allow_empty_value: _,
                            } => {
                                result.push_str(
                                    format!(
                                        "  - Query parameter `{}` was removed.\n",
                                        parameter_data.name
                                    )
                                    .as_str(),
                                );
                            }
                            Parameter::Header {
                                parameter_data,
                                style: _,
                            } => {
                                result.push_str(
                                    format!("  - Header `{}` was removed.\n", parameter_data.name)
                                        .as_str(),
                                );
                            }
                            Parameter::Path {
                                parameter_data,
                                style: _,
                            } => {
                                result.push_str(
                                    format!(
                                        "  - Path parameter `{}` was removed.\n",
                                        parameter_data.name
                                    )
                                    .as_str(),
                                );
                            }
                            Parameter::Cookie {
                                parameter_data,
                                style: _,
                            } => {
                                result.push_str(
                                    format!("  - Cookie `{}` was removed.\n", parameter_data.name)
                                        .as_str(),
                                );
                            }
                        },
                    }
                }
            }
        }

        result.push_str("\n");

        result
    }
}
