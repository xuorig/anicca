use super::common::OptionalStringDiff;
use crate::openapi::{ReferenceOr, Schema};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SchemaDiff {
    pub type_changed: Option<OptionalStringDiff>,
    pub description_changed: Option<OptionalStringDiff>,
    pub properties_added: Vec<String>,
    pub properties_removed: Vec<String>,
}

impl SchemaDiff {
    pub fn has_changes(&self) -> bool {
        self.type_changed.is_some()
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

        Self {
            type_changed: OptionalStringDiff::from_strings(
                &base_schema.schema_type,
                &head_schema.schema_type,
            ),
            description_changed: OptionalStringDiff::from_strings(
                &base_schema.description,
                &head_schema.description,
            ),
            properties_added: vec![],
            properties_removed: vec![],
        }
    }
}
