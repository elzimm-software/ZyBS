use clap::{Args, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub(crate) enum Shell {
    Bash,
    Zsh,
}

#[derive(Args, Debug)]
pub(crate) struct CompletionsArgs {
    pub(crate) shell: Shell,
}