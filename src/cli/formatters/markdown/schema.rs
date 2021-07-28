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

        if let Some(properties_diff) = &self.diff.properties_changed {
            for p in &properties_diff.added {
                result.push_str(&format!(
                    "{:indent$}- Property `{}` was added.\n",
                    "",
                    p.0,
                    indent = self.indent
                ));
            }

            for p in &properties_diff.removed {
                result.push_str(&format!(
                    "{:indent$}- Property `{}` was removed.\n",
                    "",
                    p.0,
                    indent = self.indent
                ));
            }

            for (p, diff) in &properties_diff.changed {
                result.push_str(&format!(
                    "{:indent$}- Property `{}` was changed:\n",
                    "",
                    p,
                    indent = self.indent
                ));

                let schema_diff = SchemaPrinter {
                    diff,
                    indent: self.indent + 2,
                }
                .print();
                result.push_str(&schema_diff);
            }
        }

        if let Some(items_diff) = &self.diff.items_changed {
            result.push_str(&format!(
                "{:indent$}- Items schema changed:\n",
                "",
                indent = self.indent
            ));

            let schema_diff = SchemaPrinter {
                diff: items_diff,
                indent: self.indent + 2,
            }
            .print();

            result.push_str(&schema_diff);
        }

        result
    }
}
