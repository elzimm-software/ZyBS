use crate::commands::Commands;
use crate::commands::new::NewCommands;
use clap::{Parser, Subcommand};
use std::error::Error;

mod args;
mod commands;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();
    match args.command {
        Commands::Init(_) => zybs_core::init(args.build_system),
        Commands::Build(_) => zybs_core::build(args.build_system),
        Commands::Generate(subargs) => {
            let debug = if subargs.debug {
                zybs_core::generate::Debug::True(subargs.debug_filter)
            } else {
                zybs_core::generate::Debug::False
            };
            zybs_core::generate(args.build_system, subargs.output, debug)
        }
        Commands::Peel(subargs) => zybs_core::peel(args.build_system, subargs.screen),
        Commands::Return(subargs) => zybs_core::zybs_return(args.build_system, subargs.screen),
        Commands::Tree(_) => zybs_core::tree(args.build_system),
        Commands::Screens(_) => zybs_core::screens(args.build_system),
        Commands::Show(subargs) => {
            zybs_core::show(args.build_system, subargs.screen, subargs.resolved)
        }
        Commands::Modules(_) => zybs_core::modules(args.build_system),
        Commands::Dirs(subargs) => zybs_core::dirs(args.build_system, subargs.screen),
        Commands::Validate(subargs) => {
            if subargs.zyl_only && subargs.zydoc_only {
                panic!(
                    "Cannot specify conflicting scope args.\nIf seeing this: remind me to write an actual error for this."
                );
            }
            let scope = if subargs.zyl_only {
                zybs_core::validate::Scope::ZylOnly
            } else if subargs.zydoc_only {
                zybs_core::validate::Scope::ZydocOnly
            } else {
                zybs_core::validate::Scope::All
            };
            zybs_core::validate(args.build_system, subargs.fix, subargs.strict, scope)
        }
        Commands::New(subargs) => {
            let wizard = match subargs.command {
                NewCommands::Screen { .. } => zybs_core::new::Wizard::Screen,
                NewCommands::ZydocEntry { .. } => zybs_core::new::Wizard::ZydocEntry,
            };
            zybs_core::new(args.build_system, wizard)
        }
        Commands::Fmt(_) => {zybs_core::fmt(args.build_system)}
        Commands::Completions(_) => {unimplemented!()}
    }
}
