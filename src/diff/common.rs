use crate::json_ref::{resolve, DereferenceError};
use http::uri::Uri;
use openapiv3::ReferenceOr;
use serde::{Deserialize, Serialize};

pub fn dereference<T: for<'de> Deserialize<'de>>(
    reference_or: ReferenceOr<T>,
    base_uri: Uri,
) -> Result<T, DereferenceError> {
    match reference_or {
        ReferenceOr::Reference { reference } => resolve::<T>(base_uri, reference),
        ReferenceOr::Item(i) => Ok(i),
    }
}

#[derive(Debug, Serialize)]
pub struct OptionalStringDiff {
    pub from: Option<String>,
    pub to: Option<String>,
}

impl OptionalStringDiff {
    pub fn from_strings(base: Option<String>, head: Option<String>) -> Option<Self> {
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
