use crate::diff::operations::OperationDiff;
use openapiv3::{Parameter, ReferenceOr};

pub struct OperationsPrinter<'a> {
    pub operation_diff: &'a OperationDiff,
}

impl<'a> OperationsPrinter<'a> {
    pub fn print(&self) -> String {
        let mut result = String::new();

        if !self.operation_diff.tags.added.is_empty() {
            result.push_str(
                format!(
                    "  - Tags were added: `{}`\n",
                    self.operation_diff.tags.added.join(",")
                )
                .as_str(),
            );
        }

        if !self.operation_diff.tags.removed.is_empty() {
            result.push_str(
                format!(
                    "  - Tags were removed: `{}`\n",
                    self.operation_diff.tags.removed.join(",")
                )
                .as_str(),
            );
        }

        if let Some(summary_diff) = &self.operation_diff.summary {
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
                    result.push_str(format!("  - Operation summary added: `{}`\n", to).as_str());
                }
            }
        }

        if let Some(diff) = &self.operation_diff.description {
            if let Some(_from) = &diff.from {
                if let Some(_to) = &diff.to {
                    result.push_str("  - Operation description was changed.\n");
                } else {
                    result.push_str("  - Operation description was removed.\n");
                }
            } else {
                if let Some(to) = &diff.to {
                    result
                        .push_str(format!("  - Operation description added: `{}`\n", to).as_str());
                }
            }
        }

        if let Some(diff) = &self.operation_diff.operation_id {
            if let Some(from) = &diff.from {
                if let Some(to) = &diff.to {
                    result.push_str(
                        format!("  - Operation id changed from `{}` to `{}`\n", from, to).as_str(),
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

        for param in &self.operation_diff.parameters.added {
            match param {
                ReferenceOr::Reference { reference } => {
                    result.push_str(
                        format!("  - Referenced parameter `{}` was added.\n", reference).as_str(),
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
                            format!("  - Query parameter `{}` was added.\n", parameter_data.name)
                                .as_str(),
                        );
                    }
                    Parameter::Header {
                        parameter_data,
                        style: _,
                    } => {
                        result.push_str(
                            format!("  - Header `{}` was added.\n", parameter_data.name).as_str(),
                        );
                    }
                    Parameter::Path {
                        parameter_data,
                        style: _,
                    } => {
                        result.push_str(
                            format!("  - Path parameter `{}` was added.\n", parameter_data.name)
                                .as_str(),
                        );
                    }
                    Parameter::Cookie {
                        parameter_data,
                        style: _,
                    } => {
                        result.push_str(
                            format!("  - Cookie `{}` was added.\n", parameter_data.name).as_str(),
                        );
                    }
                },
            }
        }

        for param in &self.operation_diff.parameters.removed {
            match param {
                ReferenceOr::Reference { reference } => {
                    result.push_str(
                        format!("  - Referenced parameter `{}` was removed.\n", reference).as_str(),
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
                            format!("  - Header `{}` was removed.\n", parameter_data.name).as_str(),
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
                            format!("  - Cookie `{}` was removed.\n", parameter_data.name).as_str(),
                        );
                    }
                },
            }
        }

        result
    }
}
