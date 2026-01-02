use clap::Parser;
use dotenv::dotenv;
use zeta_hash::args::Cli;
use zeta_hash::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();
    
    if let Err(e) = run(cli.cmd).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}