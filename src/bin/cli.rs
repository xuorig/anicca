use structopt::StructOpt;

fn main() {
    let app = heraclitus::cli::Heraclitus::from_args();
    app.run();
}
