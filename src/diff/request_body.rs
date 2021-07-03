use openapiv3::{ReferenceOr, RequestBody};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RequestBodyDiff {}

impl RequestBodyDiff {
    pub fn has_changes(&self) -> bool {
        true
    }

    pub fn from_request_bodies(
        base: &Option<ReferenceOr<RequestBody>>,
        head: &Option<ReferenceOr<RequestBody>>,
    ) -> Self {
        Self {}
    }
}
