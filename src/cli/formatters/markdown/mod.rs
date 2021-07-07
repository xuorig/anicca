pub mod extensions;
pub mod meta;
pub mod operations;
pub mod parameters;
pub mod paths;
pub mod request_body;
pub mod responses;
pub mod schema;

use crate::diff::Diff;
use meta::MetaPrinter;
use paths::PathsPrinter;

pub struct Printer {}

impl Printer {
    pub fn print(diff: &Diff) -> String {
        let mut result = String::new();

        result.push_str("# OpenAPI diff\n\n");

        let meta = MetaPrinter { diff: &diff }.print();
        result.push_str(&meta);

        if let Some(paths_diff) = &diff.paths {
            let paths = PathsPrinter { diff: paths_diff }.print();
            result.push_str(&paths);
        }

        result
    }
}
