use std::fmt;
use std::fs::{self, File};
use colored::*; 

#[derive(Debug)]
pub enum ErrorType {
    FileReadError,
    FileWriteError,
    FileOpenError,
    FileDataError
}

#[derive(Debug)]
pub struct CmdError {
    pub msg: String,
    pub error_type: ErrorType
}

impl CmdError {
    fn new(msg: String, error: ErrorType) -> Self {
        Self {
            msg: msg,
            error_type: error
        }
    }
}

impl fmt::Display for CmdError {
    // Add some color to our error messages
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} -> {}", self.error_type, self.msg.red())
    }
}

pub fn search_file(query: &String, file: &String) -> Result<usize, CmdError> {
    let content = fs::read_to_string(file).map_err(|e| CmdError::new(e.to_string(), ErrorType::FileReadError))?;

    // Token every word
    let words: Vec<&str> = content.split_whitespace().collect();
    let mut counter: usize = 0;
            
    for word in words {
        if word == query {
            counter += 1;
        }
    }
    Ok(counter)
}

pub fn replace_in_file(to_replace: &String, with_replace: &String, file: &String) -> Result<(), CmdError> {
    let content: String = fs::read_to_string(file).map_err(|e| CmdError::new(e.to_string(), ErrorType::FileReadError))?;
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
    
    fs::write(file, new_content).map_err(|e| CmdError::new(e.to_string(), ErrorType::FileWriteError))?;
    Ok(())
}

pub fn get_file_size_in_megabytes(file: &String) -> Result<f64, CmdError> {
    let opened_file = File::open(file).map_err(|e| CmdError::new(e.to_string(), ErrorType::FileOpenError))?;
    let file_size_in_bytes = opened_file.metadata().map_err(|e| CmdError::new(e.to_string(), ErrorType::FileOpenError))?.len();
    let file_size_in_megabytes = file_size_in_bytes as f64 / 1048576.0;

    Ok(file_size_in_megabytes.round())
}

pub fn get_file_size_in_bytes(file: &String) -> Result<u64, CmdError> {
    let opened_file = File::open(file).map_err(|e| CmdError::new(e.to_string(), ErrorType::FileOpenError))?;
    let file_size_in_bytes = opened_file.metadata().map_err(|e| CmdError::new(e.to_string(), ErrorType::FileOpenError))?.len();
    
    Ok(file_size_in_bytes)
}

pub fn get_word_count(file: &String) -> Result<usize, CmdError> {
    let content = fs::read_to_string(file).map_err(|e| CmdError::new(e.to_string(), ErrorType::FileReadError))?;
    let mut word_count: usize = 0;

    for _word in content.split_whitespace() {
        word_count += 1;
    }
    Ok(word_count)
}

pub fn get_char_count(file: &String) -> Result<usize, CmdError> {
    let content = fs::read_to_string(file).map_err(|e| CmdError::new(e.to_string(), ErrorType::FileReadError))?;
    let mut char_count: usize = 0;

    for word in content.split_whitespace() {
        char_count += word.len();
    }
    Ok(char_count)
}
