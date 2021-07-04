use super::content::ContentDiff;
use openapiv3::{ReferenceOr, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseDiff {
    pub content: Option<ContentDiff>,
}

impl ResponseDiff {
    pub fn has_changes(&self) -> bool {
        self.content.is_some()
    }

    pub fn from_responses(base: &ReferenceOr<Response>, head: &ReferenceOr<Response>) -> Self {
        let base_response = match &base {
            ReferenceOr::Item(i) => i,
            ReferenceOr::Reference { reference: _ } => {
                panic!("$ref not supported yet");
            }
        };

        let head_response = match &head {
            ReferenceOr::Item(i) => i,
            ReferenceOr::Reference { reference: _ } => {
                panic!("$ref not supported yet");
            }
        };

        let content_diff =
            ContentDiff::from_content(&base_response.content, &head_response.content);

        if content_diff.has_changes() {
            Self {
                content: Some(content_diff),
            }
        } else {
            Self { content: None }
        }
    }
}
