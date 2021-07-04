use super::schema::SchemaPrinter;
use crate::diff::responses::ResponsesDiff;

pub struct ResponsesPrinter<'a> {
    pub responses: &'a ResponsesDiff,
}

impl<'a> ResponsesPrinter<'a> {
    pub fn print(&self) -> String {
        let mut result = String::new();

        for response_code_pair in &self.responses.added {
            result.push_str(&format!(
                "  - Response with status {} was added.",
                response_code_pair.0
            ));
        }

        for response_code_pair in &self.responses.removed {
            result.push_str(&format!(
                "  - Response with status {} was removed.",
                response_code_pair.0
            ));
        }

        for (status_code, response_diff) in &self.responses.changed {
            result.push_str(&format!(
                "  - Response with status {} changed:",
                status_code
            ));

            if let Some(content_diff) = &response_diff.content {
                for media_type_pair in &content_diff.added {
                    result.push_str(&format!(
                        "    - Request body media type {} was added.",
                        media_type_pair.0
                    ));
                }

                for media_type_pair in &content_diff.removed {
                    result.push_str(&format!(
                        "    - Request body media type {} was removed.",
                        media_type_pair.0
                    ));
                }

                for (media_type, media_type_diff) in &content_diff.changed {
                    result.push_str(&format!(
                        "    - Request body media type {} changed:.",
                        media_type
                    ));

                    if let Some(schema_diff) = &media_type_diff.schema_changed {
                        let schema = SchemaPrinter { diff: &schema_diff }.print();
                        result.push_str(&schema);
                    }
                }
            }
        }

        result
    }
}
