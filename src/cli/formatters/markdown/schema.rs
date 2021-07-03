use crate::diff::schema::SchemaDiff;

pub struct SchemaPrinter<'a> {
    pub diff: &'a SchemaDiff,
}

impl<'a> SchemaPrinter<'a> {
    pub fn print(&self) -> String {
        let mut result = String::new();

        if let Some(type_change) = &self.diff.type_changed {
            result.push_str(&format!(
                "  - Schema type changed from `{}` to `{}`",
                type_change.from, type_change.to
            ));
        }

        for p in &self.diff.properties_added {
            result.push_str(&format!("  - Property `{}` was added", p));
        }

        for p in &self.diff.properties_removed {
            result.push_str(&format!("  - Property `{}` was removed", p));
        }

        result
    }
}
