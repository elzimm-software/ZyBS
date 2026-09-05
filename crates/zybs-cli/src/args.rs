use crate::commands::Commands;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // TODO: add real about and long_about
pub(crate) struct Args {
    #[arg(short, long, global = true)]
    pub(crate) build_system: Option<String>,
    #[command(subcommand)]
    pub(crate) command: Commands,
}