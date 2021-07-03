use crate::diff::responses::ResponsesDiff;

pub struct ResponsesPrinter<'a> {
    pub responses: &'a ResponsesDiff,
}

impl<'a> ResponsesPrinter<'a> {
    pub fn print(&self) -> String {
        let result = String::new();
        result
    }
}
