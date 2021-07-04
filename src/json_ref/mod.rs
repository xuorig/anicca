pub mod uri_utils;

use http::uri;
use serde::Deserialize;
use std::fs::File;
use std::str::FromStr;
use thiserror::Error;
use uri_utils::uri_join;

#[derive(Error, Debug)]
pub enum DereferenceError {
    #[error(transparent)]
    InvalidUriError(#[from] uri::InvalidUriParts),

    #[error("Unsupported ref: {0}")]
    UnsupportedRef(uri::Uri),

    #[error(transparent)]
    SerdeError(#[from] serde_yaml::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub fn resolve<'a, T: for<'de> Deserialize<'de>>(
    base_uri: uri::Uri,
    reference: String,
) -> Result<T, DereferenceError> {
    let joined = uri_join(base_uri, &reference)?;

    let file_scheme = &uri::Scheme::from_str("file").unwrap();
    let scheme = joined.scheme().unwrap_or(file_scheme);

    match scheme.as_str() {
        "file" => {
            let f = File::open(joined.to_string())?;
            let d: T = serde_yaml::from_reader(f)?;
            Ok(d)
        }
        _ => Err(DereferenceError::UnsupportedRef(joined)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openapiv3::{Schema, SchemaKind, Type};
    use uri::Uri;

    #[test]
    fn test_resolve_file() {
        let mut base_uri = String::from(env!("CARGO_MANIFEST_DIR"));
        // Trailing slash is important to allow URI join
        base_uri.push_str("/");

        let base_uri = base_uri.parse::<Uri>().unwrap();

        let schema = resolve::<Schema>(base_uri, String::from("./fixtures/schema.json"))
            .expect("Failed to deref");

        assert!(matches!(
            schema.schema_kind,
            SchemaKind::Type(Type::Object(_))
        ));
    }
}
