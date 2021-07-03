use crate::diff::schema::SchemaDiff;

pub struct SchemaPrinter<'a> {
    pub diff: &'a SchemaDiff,
}

impl<'a> SchemaPrinter<'a> {
    pub fn print(&self) -> String {
        let result = String::new();
        result
    }
}
