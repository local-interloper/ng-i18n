use clap::{Args, Parser, Subcommand};

#[derive(Args)]
pub struct UpdaterArgs {
    /// The path to messages.json or where messages.json should be created.
    /// If it doesn't exist, ng extract-i18n will be used to create it
    #[arg(long = "source")]
    pub source_path: Option<String>,

    /// Comma separated list of languages for which language files should be
    /// generated (e.g. --lang de,hr,it)
    #[arg(long = "langs", value_delimiter = ',')]
    pub target_languages: Option<Vec<String>>,

    /// Disables sorting of the source file in case you want it unmodified
    #[arg(long)]
    pub no_sort: bool,

    /// Disables extraction of i18n and possibly also creation of messages.json
    /// in case it doesn't exist
    #[arg(long)]
    pub no_extract: bool,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Update (UpdaterArgs),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ProgramArgs {
    #[command(subcommand)]
    pub command: Subcommands
}