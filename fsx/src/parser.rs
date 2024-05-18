use clap::{Args, Subcommand, Parser};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct FsxArgs {
    #[clap(subcommand)]
    pub command: Command
}

#[derive(Debug, Subcommand)]
pub enum Command {  
    Search(SearchCommand),
}

#[derive(Debug, Args)]
pub struct SearchCommand {
    pub file: String,
    pub query: String
}