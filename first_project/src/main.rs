use std::{env, fs};

enum Command {
    Search(String, String),
    Replace(String, String, String)
}

struct Error {
    msg: String
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Writes to a buffer with a custom error message
        write!(f, "{}", self.msg)
    }
}

impl Command {

    fn parse_command_args(args: &[String]) -> Result<Command, Error> {
        if args.len() < 4 {
            Err(Error { msg: "Not enough arguments given".to_string() } )
        } else {
            let command = &args[1];
            match command.as_str() {
                "search" => Ok(Command::Search(args[2].clone(), args[3].clone())),
                "replace" => Ok(Command::Replace(args[2].clone(), args[3].clone() , args[4].clone())),
                _ => Err(Error { msg: format!("{command} is not a command.") } )
                }
            } 
        }
    }

    fn execute(&self) {
        match self {
            Command::Search(query, file) => {
                println!("Searching {} for '{}'...", file, query);
                let content: String = fs::read_to_string(file).expect("Couldn't read file");
                // Token every word
                let words: Vec<&str> = content.split_whitespace().collect();
                let mut counter: u8 = 0;
        
                for word in words {
                    if word == query {
                        counter += 1;
                    }
                }

                println!("Found {} instances of '{}' in {}", counter, query, file);
            },
            Command::Replace(to_replace, with_replace,file) => {
                println!("Replacing...");

                let content: String = fs::read_to_string(file).expect("Coudln't read file");
                let mut new_content = String::new();

                for line in content.lines() {
                    for word in line.split_whitespace() {
                        if word == to_replace {
                            new_content.push_str(format!("{with_replace} ").as_str());
                        } else {
                            new_content.push_str(format!("{word} ").as_str());
                        }
                    }
                    new_content.push_str("\n");
                }

                fs::write(file, new_content).expect("Couldn't write to file");

                println!("Sucessfully replaced {} with {}", to_replace, with_replace);
            }
        };
    }
}


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
