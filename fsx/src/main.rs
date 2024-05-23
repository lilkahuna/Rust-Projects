mod parser;
mod cmd;
mod error;

use parser::{FsxArgs, Command};
use error::CmdError;
use clap::Parser;
use cmd::*;
use colored::*;

 fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
 }

fn run() -> Result<(), CmdError> {
    let args = FsxArgs::parse();
    
    match args.command {
        Command::Search(search_args) => {
            let instances = search_file(&search_args.query, &search_args.file)?;

            println!("{} instance(s) of {} found in {}", instances.to_string().green(), search_args.query.green(), search_args.file.green());
        },
        Command::Replace(replace_args) => {
            cmd::replace_in_file(&replace_args.to_replace, &replace_args.with_replace, &replace_args.file)?;
            println!("Replaced all instances of {} in {}", replace_args.to_replace.green(), replace_args.file.green());
        },
        Command::Info(info_args) => {
            let char_count = get_char_count(&info_args.file)?;
            let word_count = get_word_count(&info_args.file)?;

            let file_size = | | -> Result<(&str, String), CmdError> {
                let unit_type: &str;
                let file_size: f64;
                
                if info_args.mb {
                    file_size = get_file_size_in_megabytes(&info_args.file)?;
                    unit_type = "megabytes";
                } else {
                    file_size = get_file_size_in_bytes(&info_args.file)? as f64;
                    unit_type = "bytes"; 
                }
    
                let file_size_str = if info_args.mb {
                    format!("{:.2}", file_size) // format to 2 decimal places
                } else {
                    file_size.to_string() // no need to format, it's an integer
                };

                Ok((unit_type, file_size_str))
            };
            
            let (unit_type, file_size_str) = file_size()?;

            println!(
                "Word count: {}\nCharacter count: {}\nFile size: {} {}",
                word_count.to_string().green(),
                char_count.to_string().green(),
                file_size_str.green(),
                unit_type.green()
            );
        }
        
    }
    Ok(())
}
