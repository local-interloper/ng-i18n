use crate::core::updater::Updater;
use crate::core::utils::print_error;
use crate::types::args::{ProgramArgs, Subcommands};
use clap::Parser;
use std::process::ExitCode;

mod core;
mod types;
fn main() -> ExitCode {
    let args = ProgramArgs::parse();

    match args.command {
        Subcommands::Update(updater_args) => {
            let updater = Updater::new(updater_args);

            if let Err(err) = updater.update() {
                print_error(&err);
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
