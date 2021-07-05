use super::common::StringDiff;
use crate::openapi::{ReferenceOr, Schema};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SchemaDiff {
    pub type_changed: Option<StringDiff>,
    pub schema_kind_changed: Option<StringDiff>,
    pub properties_added: Vec<String>,
    pub properties_removed: Vec<String>,
}

impl SchemaDiff {
    pub fn has_changes(&self) -> bool {
        self.type_changed.is_some()
            || self.schema_kind_changed.is_some()
            || !self.properties_added.is_empty()
            || !self.properties_removed.is_empty()
    }

    pub fn from_schemas(base: &ReferenceOr<Schema>, head: &ReferenceOr<Schema>) -> Self {
        let base_schema = if let ReferenceOr::Item(s) = base {
            s
        } else {
            panic!("Refs are not handled yet.");
        };

        let head_schema = if let ReferenceOr::Item(s) = head {
            s
        } else {
            panic!("Refs are not handled yet.");
        };

        let mut diff = Self {
            type_changed: None,
            schema_kind_changed: None,
            properties_added: vec![],
            properties_removed: vec![],
        };

        diff
    }
}
