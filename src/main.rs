
use clap::Parser;

mod fabric;
mod ai;

#[derive(Parser)]
struct Cli {
    command: String
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    ai::run(args.command).await;
}
