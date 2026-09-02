use clap::{Args, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum Shell {
    Bash,
    Zsh,
}

#[derive(Args, Debug)]
pub(crate) struct CompletionsArgs {
    shell: Shell,
}