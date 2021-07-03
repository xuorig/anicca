use crate::diff::request_body::RequestBodyDiff;

pub struct RequestBodyPrinter<'a> {
    pub request_body: &'a RequestBodyDiff,
}

impl<'a> RequestBodyPrinter<'a> {
    pub fn print(&self) -> String {
        let result = String::new();
        result
    }
}
