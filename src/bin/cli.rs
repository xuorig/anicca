use anicca::cli::Anicca;
use structopt::StructOpt;

fn main() {
    let app = Anicca::from_args();
    app.run();
}
