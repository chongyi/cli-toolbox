use clap::Parser;

mod toolkit;
mod cli;

fn main() {
    let args = cli::Args::parse();

    args.run_toolkit()
}
