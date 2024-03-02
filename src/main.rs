mod fabric;
use clap::Parser;

mod ai;

#[derive(Parser)]
struct Cli {
    command: String
}

fn main() {
    let args = Cli::parse();
    ai::run(args.command)
}
