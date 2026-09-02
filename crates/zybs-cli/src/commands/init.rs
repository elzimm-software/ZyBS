use clap::{Args};

#[derive(Args, Debug)]
pub(crate) struct InitArgs {
    build_system: Option<String>,
}