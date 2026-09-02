use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct ValidateArgs {
    #[arg(short, long)]
    fix: bool,
    #[arg(short, long)]
    strict: bool,
    #[arg(long)]
    zyl_only: bool,
    #[arg(long)]
    zydoc_only: bool,
}