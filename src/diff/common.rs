use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Serialize)]
pub struct StringListDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

impl StringListDiff {
    pub fn from_lists(base: &Vec<String>, head: &Vec<String>) -> Self {
        let base_set: HashSet<_> = base.iter().collect();
        let added: Vec<_> = head
            .iter()
            .filter(|item| !base_set.contains(item))
            .cloned()
            .collect();

        let head_set: HashSet<_> = head.iter().collect();
        let removed: Vec<_> = base
            .iter()
            .filter(|item| !head_set.contains(item))
            .cloned()
            .collect();

        Self { added, removed }
    }

    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.removed.is_empty()
    }
}

#[derive(Debug, Serialize)]
pub struct OptionalStringDiff {
    pub from: Option<String>,
    pub to: Option<String>,
}

impl OptionalStringDiff {
    pub fn from_strings(base: &Option<String>, head: &Option<String>) -> Option<Self> {
        if base != head {
            Some(Self {
                from: base.clone(),
                to: head.clone(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StringDiff {
    pub from: String,
    pub to: String,
}

impl StringDiff {
    pub fn from_strings(base: String, head: String) -> Option<Self> {
        if base != head {
            Some(Self {
                from: base,
                to: head,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BooleanDiff {
    pub from: bool,
    pub to: bool,
}

impl BooleanDiff {
    pub fn from_bools(base: bool, head: bool) -> Option<Self> {
        if base != head {
            Some(Self {
                from: base,
                to: head,
            })
        } else {
            None
        }
    }
}
