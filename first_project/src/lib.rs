pub mod cmd {
    use std::fmt::{self, Debug};
    use std::{fs::{self, File}, io::Read};

    #[derive(PartialEq, Debug)]
    pub enum Command {
        Search(String, String),
        Replace(String, String, String),
        Info(String)
    }
    
    #[derive(PartialEq, Debug)]
    pub enum ErrorType {
        InvalidArgument,
        NotEnoughArguments,
    }

    #[derive(PartialEq)]
    pub struct Error {
        pub msg: String,
        pub error: ErrorType,
    }

    impl Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?} -> {}", self.error, self.msg)
        }
    }
    
    impl Command {
        /**
            # Example
            Creates a varient of the Command type with all valid arguments based on the subcommand in the ```args``` vector passed.

            Returns a Result containing ```Command``` or ```Error```.
            ```
            use std::env;
            use first_project::cmd::{Command, Error};

            let args: Vec<String> = env::args().collect();
            let command: Result<Command, Error> = Command::parse_command_args(&args);
            ```
         */
        pub fn parse_command_args(args: &[String]) -> Result<Command, Error> {
            if args.len() < 3 {
                Err(Error { msg: String::from("Not enough arguments given"), error: ErrorType::NotEnoughArguments } )
            } else {
                let command = &args[1];
                match command.as_str() {
                    "search" => Ok(Command::Search(args[2].clone(), args[3].clone())),
                    "replace" => Ok(Command::Replace(args[2].clone(), args[3].clone() , args[4].clone())),
                    "info" => Ok(Command::Info(args[2].clone())),
                    _ => Err(Error { msg: format!("{command} is not a command."), error: ErrorType::InvalidArgument } )
                }
            } 
        }
        
        /**
            # Example
            ```
            
            ```
         */
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
                    let file_size = opened_file.metadata().expect("Can't get metadata").len();
                    
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