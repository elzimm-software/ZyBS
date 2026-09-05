use clap::{Args};

#[derive(Args, Debug)]
pub(crate) struct InitArgs {
    pub(crate) build_system: Option<String>,
}