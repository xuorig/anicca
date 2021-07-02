pub(crate) mod diff;
pub(crate) mod docs;
pub(crate) mod markdown;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Heraclitus",
    about = "
Read the getting started guide by running:
    $ heraclitus docs

Generate a human friendly diff:
    $ heraclitus diff <base.json> <head.json>
"
)]
pub struct Heraclitus {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Open the Hercalitus docs online
    Docs(docs::DocsCommand),

    /// Diff two OpenAPI descriptions
    Diff(diff::DiffCommand),
}

impl Heraclitus {
    pub fn run(&self) {
        match &self.command {
            Command::Docs(command) => command.run(),
            Command::Diff(command) => command.run(),
        }
    }
}
