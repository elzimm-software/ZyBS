use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct ShowArgs {
    screen: String,
    #[arg(short, long)]
    resolved: bool,
}