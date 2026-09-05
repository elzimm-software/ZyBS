use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub(crate) struct NewArgs {
    #[command(subcommand)]
    pub(crate) command: NewCommands
}

#[derive(Subcommand, Debug)]
pub(crate) enum NewCommands {
    Screen {
        name: Option<String>
    },
    ZydocEntry {
        screen: Option<String>
    }
}