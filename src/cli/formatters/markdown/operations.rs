use super::parameters::ParametersPrinter;
use super::request_body::RequestBodyPrinter;
use super::responses::ResponsesPrinter;
use crate::diff::operations::OperationDiff;

pub struct OperationsPrinter<'a> {
    pub operation_diff: &'a OperationDiff,
}

impl<'a> OperationsPrinter<'a> {
    pub fn print(&self) -> String {
        let mut result = String::new();

        if !self.operation_diff.tags.added.is_empty() {
            result.push_str(
                format!(
                    "      - Tags were added: `{}`\n",
                    self.operation_diff.tags.added.join(",")
                )
                .as_str(),
            );
        }

        if !self.operation_diff.tags.removed.is_empty() {
            result.push_str(
                format!(
                    "      - Tags were removed: `{}`\n",
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
                            "      - Operation summary changed from `{}` to `{}`.\n",
                            from, to
                        )
                        .as_str(),
                    );
                } else {
                    result.push_str("      - Operation summary was removed.\n");
                }
            } else if let Some(to) = &summary_diff.to {
                result.push_str(format!("      - Operation summary added: `{}`\n", to).as_str());
            }
        }

        if let Some(diff) = &self.operation_diff.description {
            if let Some(_from) = &diff.from {
                if let Some(_to) = &diff.to {
                    result.push_str("      - Operation description was changed.\n");
                } else {
                    result.push_str("      - Operation description was removed.\n");
                }
            } else if let Some(to) = &diff.to {
                result
                    .push_str(format!("      - Operation description added: `{}`\n", to).as_str());
            }
        }

        if let Some(diff) = &self.operation_diff.operation_id {
            if let Some(from) = &diff.from {
                if let Some(to) = &diff.to {
                    result.push_str(
                        format!("      - Operation id changed from `{}` to `{}`\n", from, to)
                            .as_str(),
                    );
                } else {
                    result.push_str("      - Operation id was removed.\n");
                }
            } else if let Some(to) = &diff.to {
                result.push_str(format!("      - Operation id added: `{}`\n", to).as_str());
            }
        }

        let params = ParametersPrinter {
            parameters: &self.operation_diff.parameters,
        }
        .print();

        result.push_str(&params);

        if let Some(request_body) = &self.operation_diff.request_body {
            let request_body = RequestBodyPrinter {
                request_body: &request_body,
            }
            .print();

            result.push_str(&request_body);
        }

        let params = ResponsesPrinter {
            responses: &self.operation_diff.responses,
        }
        .print();

        result.push_str(&params);

        result
    }
}
