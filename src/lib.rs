use openapiv3::OpenAPI;
use thiserror::Error;

/// DiffError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum DiffError {
    /// Represents an invalid OpenAPI document according to the specification.
    #[error("Invalid OpenAPI document")]
    InvalidOpenAPI,

    /// Represents all cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

/// OpenAPIVersionChange represents a change in the OpenAPI specification version
/// between documents.
#[derive(Debug)]
pub struct OpenAPIVersionChange {
    from: String,
    to: String,
}

impl OpenAPIVersionChange {
    pub fn message(&self) -> String {
        format!(
            "OpenAPI specification version changed from {} to {}.",
            self.from, self.to
        )
    }
}

#[derive(Debug)]
pub enum Change {
    OpenAPIVersionChange(OpenAPIVersionChange),
}

impl Change {
    pub fn message(&self) -> String {
        match self {
            Self::OpenAPIVersionChange(c) => c.message(),
        }
    }
}

#[derive(Debug)]
pub struct Diff {
    pub changes: Vec<Change>,
}

impl Diff {
    pub fn from_changes(changes: Vec<Change>) -> Self {
        Diff { changes }
    }
}

pub fn diff(base: OpenAPI, head: OpenAPI) -> Result<Diff, DiffError> {
    let mut changes = vec![];

    if base.openapi != head.openapi {
        changes.push(Change::OpenAPIVersionChange(OpenAPIVersionChange {
            from: base.openapi,
            to: head.openapi,
        }))
    }

    Ok(Diff::from_changes(changes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openapi_version_change() {
        let mut base = OpenAPI::default();
        base.openapi = String::from("3.0.0");
        let mut head = OpenAPI::default();
        head.openapi = String::from("4.0.0");

        let result = diff(base, head);
        let diff = result.expect("Failed to diff");

        assert_eq!(diff.changes.len(), 1);
        let version_change = diff.changes.first().unwrap();

        assert_eq!(
            "OpenAPI specification version changed from 3.0.0 to 4.0.0.",
            version_change.message()
        );
    }
}
