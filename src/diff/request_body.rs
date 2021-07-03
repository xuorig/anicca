use super::content::ContentDiff;
use openapiv3::{ReferenceOr, RequestBody};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RequestBodyDiff {
    added: Option<ReferenceOr<RequestBody>>,
    removed: Option<ReferenceOr<RequestBody>>,
    content_changed: Option<ContentDiff>,
}

impl RequestBodyDiff {
    pub fn has_changes(&self) -> bool {
        true
    }

    pub fn from_request_bodies(
        base: &Option<ReferenceOr<RequestBody>>,
        head: &Option<ReferenceOr<RequestBody>>,
    ) -> Self {
        match base {
            Some(base) => match head {
                Some(head) => {
                    let derefed_base = match &base {
                        ReferenceOr::Item(bc) => bc,
                        ReferenceOr::Reference { reference: _ } => {
                            panic!("$ref not supported yet");
                        }
                    };

                    let derefed_head = match &head {
                        ReferenceOr::Item(pi) => pi,
                        ReferenceOr::Reference { reference: _ } => {
                            panic!("$ref not supported yet");
                        }
                    };

                    let mut diff = Self {
                        added: None,
                        removed: None,
                        content_changed: None,
                    };

                    let content_diff =
                        ContentDiff::from_content(&derefed_base.content, &derefed_head.content);

                    if content_diff.has_changes() {
                        diff.content_changed = Some(content_diff);
                    }

                    diff
                }
                None => Self {
                    added: None,
                    removed: Some(base.clone()),
                    content_changed: None,
                },
            },
            None => match head {
                Some(head) => Self {
                    added: Some(head.clone()),
                    removed: None,
                    content_changed: None,
                },
                None => Self {
                    added: None,
                    removed: None,
                    content_changed: None,
                },
            },
        }
    }
}
