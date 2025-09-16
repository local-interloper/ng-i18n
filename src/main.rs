use crate::core::merger::Merger;
use crate::core::updater::Updater;
use crate::core::utils::print_error;
use crate::types::args::{ProgramArgs, Subcommands};
use clap::Parser;
use std::process::ExitCode;

mod core;
mod types;
fn main() -> ExitCode {
    let args = ProgramArgs::parse();

    let result = match args.command {
        Subcommands::Update(updater_args) => Updater::new(updater_args).update(),
        Subcommands::Merge(merger_args) => Merger::new(merger_args).merge()
    };

    if let Err(err) = result {
        print_error(&err);
        return ExitCode::FAILURE
    }

    ExitCode::SUCCESS
}
