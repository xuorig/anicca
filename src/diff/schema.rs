use super::common::StringDiff;
use openapiv3::{ObjectType, ReferenceOr, Schema, SchemaKind, Type};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SchemaDiff {
    type_changed: Option<StringDiff>,
    schema_kind_changed: Option<StringDiff>,
    properties_added: Vec<String>,
    properties_removed: Vec<String>,
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

        match &base_schema.schema_kind {
            SchemaKind::Type(bt) => {
                if let SchemaKind::Type(ht) = &head_schema.schema_kind {
                    if bt != ht {
                        diff.type_changed = StringDiff::from_strings(
                            Self::schema_type_to_string(bt),
                            Self::schema_type_to_string(ht),
                        );
                    }

                    match bt {
                        Type::String(_) => {}
                        Type::Number(_) => {}
                        Type::Integer(_) => {}
                        Type::Object(object_schema) => {
                            if let Type::Object(head_object_schema) = ht {
                                let object_diff =
                                    ObjectDiff::from_schemas(object_schema, head_object_schema);
                                diff.properties_added = object_diff.properties_added;
                                diff.properties_removed = object_diff.properties_removed;
                            }
                        }
                        Type::Array(_) => {}
                        Type::Boolean {} => {}
                    }
                } else {
                    diff.schema_kind_changed = StringDiff::from_strings(
                        Self::schema_kind_to_string(&base_schema.schema_kind),
                        Self::schema_kind_to_string(&head_schema.schema_kind),
                    )
                }
            }
            _ => {}
        }

        diff
    }

    pub fn schema_kind_to_string(kind: &SchemaKind) -> String {
        match kind {
            SchemaKind::Type(_) => String::from("Type"),
            SchemaKind::OneOf { one_of: _ } => String::from("OneOf"),
            SchemaKind::AllOf { all_of: _ } => String::from("AllOf"),
            SchemaKind::AnyOf { any_of: _ } => String::from("AnyOf"),
            SchemaKind::Any(_) => String::from("Any"),
        }
    }

    pub fn schema_type_to_string(t: &Type) -> String {
        match t {
            Type::String(_) => String::from("string"),
            Type::Number(_) => String::from("number"),
            Type::Integer(_) => String::from("integer"),
            Type::Object(_) => String::from("object"),
            Type::Array(_) => String::from("array"),
            Type::Boolean {} => String::from("boolean"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ObjectDiff {
    properties_added: Vec<String>,
    properties_removed: Vec<String>,
}

impl ObjectDiff {
    pub fn has_changes(&self) -> bool {
        // TODO
        true
    }

    pub fn from_schemas(base: &ObjectType, head: &ObjectType) -> Self {
        let mut properties_added = vec![];
        let mut properties_removed = vec![];

        for (property, schema) in &base.properties {
            let head_property = head.properties.get(property);

            if let Some(head_property) = head_property {
                // Diff property
            } else {
                properties_removed.push(property.clone());
            }
        }

        for (property, schema) in &head.properties {
            let base_property = base.properties.get(property);

            if let Some(head_property) = base_property {
                // NOOP
            } else {
                properties_added.push(property.clone());
            }
        }

        Self {
            properties_added,
            properties_removed,
        }
    }
}
