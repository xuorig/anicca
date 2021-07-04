use super::schema::SchemaPrinter;
use crate::diff::request_body::RequestBodyDiff;

pub struct RequestBodyPrinter<'a> {
    pub request_body: &'a RequestBodyDiff,
}

impl<'a> RequestBodyPrinter<'a> {
    pub fn print(&self) -> String {
        let mut result = String::new();

        if self.request_body.added.is_some() {
            result.push_str("Request body was added.\n");
        }

        if self.request_body.removed.is_some() {
            result.push_str("Request body was added.\n");
        }

        if let Some(content_diff) = &self.request_body.content_changed {
            for media_type_pair in &content_diff.added {
                result.push_str(&format!(
                    "  - Request body media type {} was added.\n",
                    media_type_pair.0
                ));
            }

            for media_type_pair in &content_diff.removed {
                result.push_str(&format!(
                    "  - Request body media type {} was removed.\n",
                    media_type_pair.0
                ));
            }

            for (media_type, media_type_diff) in &content_diff.changed {
                result.push_str(&format!(
                    "  - Request body media type {} changed:.\n",
                    media_type
                ));

                if let Some(schema_diff) = &media_type_diff.schema_changed {
                    let schema = SchemaPrinter {
                        diff: &schema_diff,
                        indent: 4,
                    }
                    .print();
                    result.push_str(&schema);
                }
            }
        }

        result
    }
}
