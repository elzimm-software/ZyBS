use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct PeelArgs {
    pub(crate) screen: String,
}