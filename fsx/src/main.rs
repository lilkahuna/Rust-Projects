use fsx::parser::{FsxArgs, Command};
use fsx::cmd;
use clap::Parser;


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
    let args = FsxArgs::parse();
    
    match args.command {
        Command::Search(search_args) => {
            let instances = cmd::search_file(&search_args.query, &search_args.file);
            println!("{} instances of '{}' found in {}", instances, search_args.query, search_args.file);
        }
    }

}
