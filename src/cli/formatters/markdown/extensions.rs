use crate::diff::extensions::ExtensionsDiff;

pub struct ExtensionsPrinter<'a> {
    pub extensions: &'a ExtensionsDiff,
    pub indent: usize,
}

impl<'a> ExtensionsPrinter<'a> {
    pub fn print(&self) -> String {
        let mut result = String::new();

        for p in &self.extensions.added {
            result.push_str(&format!(
                "{:indent$}- Extension `{}` was added.\n",
                "",
                p.0,
                indent = self.indent
            ));
        }

        result
    }
}
