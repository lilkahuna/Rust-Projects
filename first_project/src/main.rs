use std::fs::File;
use std::io::{Read, Result, Write};

// Definition of the example_mod mod(looks for a file named after the module)
mod example_mod;
// Re-exports it for public use
pub use crate::example_mod::hosting;

/** IMPORTANT INFORMATION:
 * Unsigned means only positive(0-255), while signed means positive or negative(-128-127)
 * A signed integer uses the first bit to store sign information. 0 for positive and vice versa.
 * 8 bits is a byte
 * A bit stores 0 or 1
 * i32 is the default integer type of rust
 * Associated functions belong to the type, not the instance
 * Structs, enums, pattern matching, traits ,and impl blocks
 * Vectors are like lists
 * Rust will panic in order to protect from buffer overread
 * :? is a formatting directive
 * traits are shared behaviors
 * ? unwraps the Result enum and handles it
 */
fn main() -> Result<()> {    
    println!("Reading image...");

    let mut file = File::open("Image.png")?;
    
    // Read the contents of the file into a buffer
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    // Print out the bytes
    for byte in &buffer {
        print!("{} ", byte);
    }

    // Sucessful run
    Ok(())
}


