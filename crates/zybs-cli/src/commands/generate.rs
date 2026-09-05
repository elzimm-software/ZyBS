use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub(crate) struct GenerateArgs {
    #[arg(short, long)]
    pub(crate) output: Option<PathBuf>,
    #[arg(short, long)]
    pub(crate) debug: bool,
    #[arg(short = 'F', long, value_delimiter = ',')]
    pub(crate) debug_filter: Vec<String>,
}
