use crate::diff::diff_json;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct DiffCommand {
    /// The base OpenAPI description
    #[structopt(short = "b", long = "base-file", parse(from_os_str))]
    base: PathBuf,

    /// The head OpenAPI description to be compared with base
    #[structopt(short = "h", long = "head-file", parse(from_os_str))]
    head: PathBuf,
}

impl DiffCommand {
    pub fn run(&self) {
        let res = diff_json(self.base.clone(), self.head.clone());

        match res {
            Ok(diff) => {
                let json =
                    serde_json::to_string_pretty(&diff).expect("Could not serialize diff to JSON");
                println!("{}", json)
            }
            Err(e) => {
                eprintln!("Error: {}", e)
            }
        }
    }
}
