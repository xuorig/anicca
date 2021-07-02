use crate::diff::diff_json;
use clap::arg_enum;
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

    /// Output format for the diff
    #[structopt(short = "f", long = "format", default_value = "default", possible_values = &Format::variants(), case_insensitive = true)]
    format: Format,
}

arg_enum! {
    #[derive(Debug)]
    pub enum Format {
        Default,
        JSON,
        YAML,
    }
}

impl DiffCommand {
    pub fn run(&self) {
        let res = diff_json(self.base.clone(), self.head.clone());

        match res {
            Ok(diff) => match self.format {
                Format::JSON => {
                    let json = serde_json::to_string_pretty(&diff)
                        .expect("Could not serialize diff to JSON");
                    println!("{}", json);
                }
                Format::YAML => {
                    let yaml =
                        serde_yaml::to_string(&diff).expect("Could not serialize diff to YAML");
                    println!("{}", yaml);
                }
                Format::Default => {
                    let json = serde_json::to_string_pretty(&diff)
                        .expect("Could not serialize diff to JSON");
                    println!("{}", json);
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e)
            }
        }
    }
}
