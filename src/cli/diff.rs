use super::formatters::markdown::Printer;
use crate::diff::diff_files;
use clap::arg_enum;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct DiffCommand {
    /// The base OpenAPI description
    #[structopt(parse(from_os_str))]
    base: PathBuf,

    /// The head OpenAPI description to be compared with base
    #[structopt(parse(from_os_str))]
    head: PathBuf,

    /// Output format for the diff
    #[structopt(short = "f", long = "format", default_value = "markdown", possible_values = &Format::variants(), case_insensitive = true)]
    format: Format,
}

arg_enum! {
    #[derive(Debug)]
    pub enum Format {
        Markdown,
        Json,
        Yaml,
    }
}

impl DiffCommand {
    pub fn run(&self) {
        let res = diff_files(self.base.clone(), self.head.clone());

        match res {
            Ok(diff) => match self.format {
                Format::Json => {
                    let json = serde_json::to_string_pretty(&diff)
                        .expect("Could not serialize diff to JSON");
                    println!("{}", json);
                }
                Format::Yaml => {
                    let yaml =
                        serde_yaml::to_string(&diff).expect("Could not serialize diff to YAML");
                    println!("{}", yaml);
                }
                Format::Markdown => {
                    let md = Printer::print(&diff);
                    println!("{}", md);
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e)
            }
        }
    }
}
