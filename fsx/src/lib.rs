pub mod parser;
pub mod cmd;

pub mod test {
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
            Creates a varient of the Command type with all valid arguments based on the subcommand in the ```args``` vector passed.

            Returns a Result containing ```Command``` or ```Error```.

            # Example
            
            ```
            use std::env;
            use fsx::cmd::{Command, Error};

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
            Consumes the ```self``` value and executes functionality depending on the varient.

            # Example
            ```
            use std::env;
            use fsx::cmd::{Command, Error};

            let args: Vec<String> = env::args().collect();
            let command: Result<Command, Error> = Command::parse_command_args(&args);
            command.execute();
            ```
         */
        pub fn execute(&self) {
            match self {
                Command::Search(query, file) => {
                    println!("Searching {} for '{}'...", file, query);
                    
    
                    println!("Found {} instances of '{}' in {}", counter, query, file);
                },
                Command::Replace(to_replace, with_replace,file) => {
                    println!("Replacing...");
    
                    
    
                    println!("Sucessfully replaced {} with {}", to_replace, with_replace);
                },
                Command::Info(file) => {
                    
                    
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