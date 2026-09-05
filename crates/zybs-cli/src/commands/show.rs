use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct ShowArgs {
    pub(crate) screen: String,
    #[arg(short, long)]
    pub(crate) resolved: bool,
}