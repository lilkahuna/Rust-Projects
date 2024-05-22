use clap::{Args, Subcommand, Parser,};

#[derive(Debug, Parser)]
#[clap(version, about = "File searching and extras!")]
pub struct FsxArgs {
    #[clap(subcommand)]
    pub command: Command
}

#[derive(Debug, Subcommand)]
pub enum Command { 
    /// Search for a word in a file 
    Search(SearchCommand),
    /// Replace a word in a file
    Replace(ReplaceCommand),
    /// Give common information on a file
    Info(InfoCommand)
}

#[derive(Debug, Args)]
pub struct SearchCommand {
    pub file: String,
    pub query: String
}

#[derive(Debug, Args)]
pub struct ReplaceCommand {
    pub file: String,
    pub to_replace: String,
    pub with_replace: String
}

#[derive(Debug, Args)]
pub struct InfoCommand {
    pub file: String,
    #[arg(short, long)]
    /// Display megabytes instead of bytes
    pub mb: bool
}