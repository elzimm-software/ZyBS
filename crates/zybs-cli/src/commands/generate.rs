use std::path::PathBuf;
use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct GenerateArgs {
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(short, long)]
    debug: bool,
    #[arg(short = 'F', long, value_delimiter = ',')]
    debug_filter: Vec<String>
}