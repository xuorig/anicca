use super::content::ContentDiff;
use crate::openapi::{ReferenceOr, RequestBody};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RequestBodyDiff {
    pub added: Option<ReferenceOr<RequestBody>>,
    pub removed: Option<ReferenceOr<RequestBody>>,
    pub content_changed: Option<ContentDiff>,
}

impl RequestBodyDiff {
    pub fn has_changes(&self) -> bool {
        self.added.is_some() || self.removed.is_some() || self.content_changed.is_some()
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
