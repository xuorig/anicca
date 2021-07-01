use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct DocsCommand {}

impl DocsCommand {
    pub fn run(&self) {
        println!("TODO");
    }
}
