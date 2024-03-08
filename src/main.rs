use clap::Parser;
use dotenv::dotenv;

mod ai;
mod cmd;
mod fabric;

#[derive(Parser)]
struct Cli {
    command: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Cli::parse();
    if args.command == "list" {
        cmd::list();
    } else {
        ai::run(args.command).await;
    }
}
