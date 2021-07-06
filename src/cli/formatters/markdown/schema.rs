use crate::diff::schema::SchemaDiff;

pub struct SchemaPrinter<'a> {
    pub diff: &'a SchemaDiff,
    pub indent: usize,
}

impl<'a> SchemaPrinter<'a> {
    pub fn print(&self) -> String {
        let mut result = String::new();

        if let Some(type_change) = &self.diff.type_changed {
            result.push_str(&format!(
                "{:indent$}- Schema type changed from `{}` to `{}`.\n",
                "",
                type_change.from.clone().unwrap_or("null".into()),
                type_change.to.clone().unwrap_or("null".into()),
                indent = self.indent,
            ));
        }

        for p in &self.diff.properties_added {
            result.push_str(&format!(
                "{:indent$}- Property `{}` was added.\n",
                "",
                p,
                indent = self.indent
            ));
        }

        for p in &self.diff.properties_removed {
            result.push_str(&format!(
                "{:indent$}- Property `{}` was removed.\n",
                "",
                p,
                indent = self.indent
            ));
        }

        result
    }
}
