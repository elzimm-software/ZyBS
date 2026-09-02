use clap::{Parser, Subcommand};

mod commands;
mod args;

fn main() {
    let args = args::Args::parse();
    println!("{:?}", args);
}
