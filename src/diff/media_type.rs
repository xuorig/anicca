use super::schema::SchemaDiff;
use openapiv3::{MediaType, ReferenceOr, Schema};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MediaTypeDiff {
    pub schema_changed: Option<SchemaDiff>,
    pub schema_added: Option<ReferenceOr<Schema>>,
    pub schema_removed: Option<ReferenceOr<Schema>>,
}

impl MediaTypeDiff {
    pub fn has_changes(&self) -> bool {
        self.schema_changed.is_some()
    }

    pub fn from_media_types(base: &MediaType, head: &MediaType) -> Self {
        let mut diff = Self {
            schema_changed: None,
            schema_removed: None,
            schema_added: None,
        };

        match &base.schema {
            Some(base_schema) => match &head.schema {
                Some(head_schema) => {
                    let schema_diff = SchemaDiff::from_schemas(&base_schema, &head_schema);

                    if schema_diff.has_changes() {
                        diff.schema_changed = Some(schema_diff);
                    }
                }
                None => {
                    diff.schema_removed = Some(base_schema.clone());
                }
            },
            None => match &head.schema {
                Some(head_schema) => {
                    diff.schema_added = Some(head_schema.clone());
                }
                None => {}
            },
        }

        diff
    }
}
