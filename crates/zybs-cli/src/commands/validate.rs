use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct ValidateArgs {
    #[arg(short, long)]
    pub(crate) fix: bool,
    #[arg(short, long)]
    pub(crate) strict: bool,
    #[arg(long)]
    pub(crate) zyl_only: bool,
    #[arg(long)]
    pub(crate) zydoc_only: bool,
}
