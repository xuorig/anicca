use crate::diff::Diff;

pub struct MetaPrinter<'a> {
    pub diff: &'a Diff,
}

impl<'a> MetaPrinter<'a> {
    pub fn print(&self) -> String {
        let mut result = String::new();

        result.push_str("## General\n\n");

        if let Some(version_change) = &self.diff.version {
            result.push_str(
                format!(
                    "  - OpenAPI spec version changed from `{}` to `{}`.\n",
                    version_change.from, version_change.to
                )
                .as_str(),
            );
        }

        result.push('\n');

        result
    }
}
