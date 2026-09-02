mod build;
mod cmd_return;
mod completions;
mod dirs;
mod fmt;
mod generate;
mod init;
mod modules;
mod new;
mod peel;
mod screens;
mod show;
mod tree;
mod validate;

use crate::commands::{
    build::BuildArgs, cmd_return::ReturnArgs, completions::CompletionsArgs, dirs::DirsArgs,
    fmt::FmtArgs, generate::GenerateArgs, init::InitArgs, modules::ModulesArgs, new::NewArgs,
    peel::PeelArgs, screens::ScreensArgs, show::ShowArgs, tree::TreeArgs, validate::ValidateArgs,
};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Scaffold a new project in the current directory
    Init(InitArgs),
    /// Validate Zyfile, generate output files, and invoke the build system
    Build(BuildArgs),
    /// Same as Build, but does not invoke build system
    Generate(GenerateArgs),
    /// Remove a screen from the active set
    Peel(PeelArgs),
    /// Return a screen to the active set
    Return(ReturnArgs),
    /// Display the full screen tree
    Tree(TreeArgs),
    /// Display list of all screens with metadata
    Screens(ScreensArgs),
    /// Display all information for given screen
    Show(ShowArgs),
    /// Display list of all modules with metadata
    Modules(ModulesArgs),
    /// Display directory dependencies for given screen
    Dirs(DirsArgs),
    /// Lint ZyBS files
    Validate(ValidateArgs),
    /// Start configuration wizard for given item
    New(NewArgs),
    /// Format .zyl and .zydoc files
    Fmt(FmtArgs),
    /// Generate shell completions
    Completions(CompletionsArgs),
}
