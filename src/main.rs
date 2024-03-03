use clap::Parser;
use dotenv::dotenv;

mod fabric;
mod ai;

#[derive(Parser)]
struct Cli {
    command: String
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Cli::parse();
    if args.command == "list" {
        fabric::list();
    } else {
        ai::run(args.command).await;
    }
}
