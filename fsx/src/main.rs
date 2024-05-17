use std::env;
use fsx::cmd::{Command, Error};

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
fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();

    let command = Command::parse_command_args(&args)?;
    command.execute();
    
    Ok(())
}
