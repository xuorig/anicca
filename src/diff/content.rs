use super::media_type::MediaTypeDiff;
use crate::openapi::MediaType;
use std::collections::BTreeMap;
use serde::Serialize;
use std::collections::HashMap;

pub type MediaTypePair = (String, MediaType);

#[derive(Debug, Serialize)]
pub struct ContentDiff {
    pub added: Vec<MediaTypePair>,
    pub removed: Vec<MediaTypePair>,
    pub changed: HashMap<String, MediaTypeDiff>,
}

impl ContentDiff {
    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.removed.is_empty() || !self.changed.is_empty()
    }

    pub fn from_content(
        base: &BTreeMap<String, MediaType>,
        head: &BTreeMap<String, MediaType>,
    ) -> Self {
        let mut media_types_added = vec![];
        let mut media_types_removed = vec![];
        let mut media_types_changed: HashMap<String, MediaTypeDiff> = HashMap::new();

        for (media_type, media_type_definition) in base {
            match head.get(media_type) {
                Some(head_media_type) => {
                    let media_type_diff =
                        MediaTypeDiff::from_media_types(&media_type_definition, &head_media_type);

                    if media_type_diff.has_changes() {
                        media_types_changed.insert(media_type.clone(), media_type_diff);
                    }
                }
                None => {
                    media_types_removed.push((media_type.clone(), media_type_definition.clone()))
                }
            }
        }

        for (media_type, media_type_definition) in head {
            match base.get(media_type) {
                Some(_) => {}
                None => media_types_added.push((media_type.clone(), media_type_definition.clone())),
            }
        }

        Self {
            added: media_types_added,
            removed: media_types_removed,
            changed: media_types_changed,
        }
    }
}
