pub(crate) mod diff;
pub(crate) mod formatters;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Anicca")]
pub struct Anicca {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Diff two OpenAPI descriptions
    Diff(diff::DiffCommand),
}

impl Anicca {
    pub fn run(&self) {
        match &self.command {
            Command::Diff(command) => command.run(),
        }
    }
}
