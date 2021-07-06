use super::common::{OptionalStringDiff, StringListDiff};
use crate::openapi::{ReferenceOr, Schema};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, Serialize, Default)]
pub struct SchemaDiff {
    pub type_changed: Option<OptionalStringDiff>,
    pub description_changed: Option<OptionalStringDiff>,
    pub format_changed: Option<OptionalStringDiff>,
    pub required_changed: Option<StringListDiff>,
    pub properties_changed: Option<PropertiesDiff>,
    pub enum_changed: Option<EnumDiff>,
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

        let mut diff = Self::default();

        let enum_diff = EnumDiff::from_schemas(&base_schema, &head_schema);
        if enum_diff.has_changes() {
            diff.enum_changed = Some(enum_diff);
        }

        let required_diff = StringListDiff::from_lists(
            &base_schema.required.clone().unwrap_or_else(|| vec![]),
            &head_schema.required.clone().unwrap_or_else(|| vec![]),
        );
        if required_diff.has_changes() {
            diff.required_changed = Some(required_diff);
        }

        let properties_diff = PropertiesDiff::from_schemas(&base_schema, &head_schema);
        if properties_diff.has_changes() {
            diff.properties_changed = Some(properties_diff);
        }

        diff.type_changed =
            OptionalStringDiff::from_strings(&base_schema.schema_type, &head_schema.schema_type);
        diff.description_changed =
            OptionalStringDiff::from_strings(&base_schema.description, &head_schema.description);
        diff.format_changed =
            OptionalStringDiff::from_strings(&base_schema.format, &head_schema.format);

        diff
    }
}

#[derive(Debug, Serialize, Default)]
pub struct EnumDiff {
    pub added: bool,
    pub removed: bool,
    pub values_added: Vec<Option<String>>,
    pub values_removed: Vec<Option<String>>,
}

impl EnumDiff {
    pub fn has_changes(&self) -> bool {
        self.added
            || self.removed
            || !self.values_added.is_empty()
            || !self.values_removed.is_empty()
    }

    pub fn from_schemas(base: &Schema, head: &Schema) -> Self {
        match &base.enum_values {
            Some(base_enum) => match &head.enum_values {
                Some(head_enum) => {
                    let base_set: HashSet<_> = base_enum.iter().collect();
                    let added: Vec<_> = head_enum
                        .iter()
                        .filter(|item| !base_set.contains(item))
                        .cloned()
                        .collect();

                    let head_set: HashSet<_> = head_enum.iter().collect();
                    let removed: Vec<_> = base_enum
                        .iter()
                        .filter(|item| !head_set.contains(item))
                        .cloned()
                        .collect();

                    Self {
                        added: false,
                        removed: false,
                        values_added: added,
                        values_removed: removed,
                    }
                }
                None => EnumDiff {
                    added: false,
                    removed: true,
                    values_removed: vec![],
                    values_added: vec![],
                },
            },
            None => match head.enum_values {
                Some(_) => EnumDiff {
                    added: true,
                    removed: false,
                    values_removed: vec![],
                    values_added: vec![],
                },
                None => EnumDiff::default(),
            },
        }
    }
}

type Property = (String, Schema);

#[derive(Debug, Serialize, Default)]
pub struct PropertiesDiff {
    pub added: Vec<Property>,
    pub removed: Vec<Property>,
    pub changed: HashMap<String, SchemaDiff>,
}

impl PropertiesDiff {
    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.removed.is_empty() || !self.changed.is_empty()
    }

    pub fn from_schemas(base: &Schema, head: &Schema) -> Self {
        let mut added = vec![];
        let mut removed = vec![];
        let mut changed: HashMap<String, SchemaDiff> = HashMap::new();

        let base_properties = base
            .properties
            .clone()
            .unwrap_or_else(|| BTreeMap::default());
        let head_properties = head
            .properties
            .clone()
            .unwrap_or_else(|| BTreeMap::default());

        for (property_name, schema) in &base_properties {
            match head_properties.get(property_name) {
                Some(head_property) => {
                    let schema_diff = SchemaDiff::from_schemas(
                        &ReferenceOr::Item(schema.clone()),
                        &ReferenceOr::Item(head_property.clone()),
                    );
                    if schema_diff.has_changes() {
                        changed.insert(property_name.clone(), schema_diff);
                    }
                }
                None => removed.push((property_name.clone(), schema.clone())),
            }
        }

        for (property_name, schema) in &head_properties {
            match base_properties.get(property_name) {
                Some(_) => {}
                None => added.push((property_name.clone(), schema.clone())),
            }
        }

        Self {
            added,
            removed,
            changed,
        }
    }
}
