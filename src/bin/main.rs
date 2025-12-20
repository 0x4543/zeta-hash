use clap::Parser;
use zeta_hash::args::Cli;
use zeta_hash::run;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli.cmd) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}