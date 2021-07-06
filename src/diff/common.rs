use serde::Serialize;

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
