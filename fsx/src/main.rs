use fsx::parser::{FsxArgs, Command};
use fsx::cmd;
use clap::{self ,Parser};
use colored::*;

/* IMPORTANT INFORMATION:
 * Unsigned means only positive(0-255), while signed means positive or negative(-128-127)
 * A signed integer uses the first bit to store sign information. 0 for positive and vice versa.
 * 8 bits is a byte
 * A bit stores 0 or 1
 * i32 is the default integer type of rust
 * Associated functions belong to the type, not the instance
 * Structs, enums, pattern matching, traits, and impl blocks
 * Vectors are like lists
 * Rust will panic in order to protect from buffer overread
 * :? is a formatting directive
 * traits are shared behaviors
 * ? unwraps the Result enum and handles it
 * Iterators create elements on the fly(lazy loading), making them more memory efficient
 */
 fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
 }

fn run() -> Result<(), cmd::CmdError> {
    let args = FsxArgs::parse();
    
    match args.command {
        Command::Search(search_args) => {
            let instances = cmd::search_file(&search_args.query, &search_args.file)?;

            println!("{} instance(s) of {} found in {}", instances.to_string().green(), search_args.query.green(), search_args.file.green());
        },
        Command::Replace(replace_args) => {
            cmd::replace_in_file(&replace_args.to_replace, &replace_args.with_replace, &replace_args.file)?;
            println!("Replaced all instances of {} in {}", replace_args.to_replace.green(), replace_args.file.green());
        },
        Command::Info(info_args) => {
            let char_count = cmd::get_char_count(&info_args.file)?;
            let word_count = cmd::get_word_count(&info_args.file)?;
            let file_size: f64;
            let unit_type: &str;

            if info_args.mb {
                file_size = cmd::get_file_size_in_megabytes(&info_args.file)?;
                unit_type = "megabytes";
            } else {
                file_size = cmd::get_file_size_in_bytes(&info_args.file)? as f64;
                unit_type = "bytes"; 
            }

            let file_size_str = if info_args.mb {
                format!("{:.2}", file_size) // format to 2 decimal places
            } else {
                file_size.to_string() // no need to format, it's an integer
            };

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
