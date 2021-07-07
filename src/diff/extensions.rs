use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

pub type ExtensionKeyValue = (String, serde_json::Value);

#[derive(Debug, Serialize)]
pub struct ExtensionsDiff {
    pub added: Vec<ExtensionKeyValue>,
    pub removed: Vec<ExtensionKeyValue>,
    pub changed: HashMap<String, ExtensionDiff>,
}

impl ExtensionsDiff {
    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.removed.is_empty() || !self.changed.is_empty()
    }

    pub fn from_extensions(
        base: &BTreeMap<String, serde_json::Value>,
        head: &BTreeMap<String, serde_json::Value>,
    ) -> Self {
        let mut extensions_added = vec![];
        let mut extensions_removed = vec![];
        let mut extensions_changed: HashMap<String, ExtensionDiff> = HashMap::new();

        for (extension_key, extension_value) in base {
            match head.get(extension_key) {
                Some(head_extension_value) => {
                    let extension_diff =
                        ExtensionDiff::from_values(&extension_value, &head_extension_value);

                    if extension_diff.has_changes() {
                        extensions_changed.insert(extension_key.clone(), extension_diff);
                    }
                }
                None => extensions_removed.push((extension_key.clone(), extension_value.clone())),
            }
        }

        for (extension_key, extension_value) in head {
            match base.get(extension_key) {
                Some(_) => {}
                None => extensions_added.push((extension_key.clone(), extension_value.clone())),
            }
        }

        ExtensionsDiff {
            added: extensions_added,
            removed: extensions_removed,
            changed: extensions_changed,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ExtensionDiff {
    from: serde_json::Value,
    to: serde_json::Value,
}

impl ExtensionDiff {
    pub fn has_changes(&self) -> bool {
        self.from != self.to
    }

    pub fn from_values(base: &serde_json::Value, head: &serde_json::Value) -> Self {
        Self {
            from: base.clone(),
            to: head.clone(),
        }
    }
}
