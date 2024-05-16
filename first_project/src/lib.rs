pub mod cmd {
    use std::{fs::{self, File}, io::Read, os::windows::fs::MetadataExt};

    #[derive(Debug, PartialEq)]
    pub enum Command {
        Search(String, String),
        Replace(String, String, String),
        Info(String)
    }
    
    pub struct Error {
        msg: String
    }
    
    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Writes to a buffer with a custom error message
            write!(f, "{}", self.msg)
        }
    }
    
    impl Command {
    
        pub fn parse_command_args(args: &[String]) -> Result<Command, Error> {
            if args.len() < 3 {
                Err(Error { msg: "Not enough arguments given".to_string() } )
            } else {
                let command = &args[1];
                match command.as_str() {
                    "search" => Ok(Command::Search(args[2].clone(), args[3].clone())),
                    "replace" => Ok(Command::Replace(args[2].clone(), args[3].clone() , args[4].clone())),
                    "info" => Ok(Command::Info(args[2].clone())),
                    _ => Err(Error { msg: format!("{command} is not a command.") } )
                }
            } 
        }
        
    
        pub fn execute(&self) {
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
                },
                Command::Info(file) => {
                    let mut opened_file = File::open(file).expect("Can't open file");
                    let file_size = opened_file.metadata().expect("Can't get metadata").file_size();
                    
                    let mut content = String::new();
                    opened_file.read_to_string(&mut content).expect("Can't read file");
                    
                    // Token every word
                    let words: Vec<&str> = content.split_whitespace().collect();
    
                    let word_count = words.len();
                    let mut character_count: usize = 0;
    
                    for word in words {
                        character_count += word.len();
                    }
                    
                    println!(
                    "
                    Info on {}:
    
                    Word count: {}
                    Character count: {}
                    File size: {} bytes
                    ", file, word_count, character_count, file_size
                    );
                }
            }
        }
    }
    
    
}