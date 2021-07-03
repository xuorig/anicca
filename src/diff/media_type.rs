use openapiv3::MediaType;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MediaTypeDiff {}

impl MediaTypeDiff {
    pub fn has_changes(&self) -> bool {
        true
    }

    pub fn from_media_types(base: &MediaType, head: &MediaType) -> Self {
        Self {}
    }
}
