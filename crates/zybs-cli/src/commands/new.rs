use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub(crate) struct NewArgs {
    #[command(subcommand)]
    command: NewCommands
}

#[derive(Subcommand, Debug)]
enum NewCommands {
    Screen {
        name: Option<String>
    },
    ZydocEntry {
        screen: Option<String>
    }
}