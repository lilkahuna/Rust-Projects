use std::fmt::{self, Debug};
use colored::*; 

#[derive(Debug)]
pub enum ErrorType {
    FileReadError,
    FileWriteError,
    FileOpenError,
}

#[derive(Debug)]
pub struct CmdError {
    pub msg: String,
    pub error_type: ErrorType
}

impl CmdError {
    pub fn new(msg: String, error: ErrorType) -> Self {
        Self {
            msg: msg,
            error_type: error
        }
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::FileReadError => write!(f, "FileReadError"),
            ErrorType::FileWriteError => write!(f, "FileWriteError"),
            ErrorType::FileOpenError => write!(f, "FileOpenError")
        }
    }
}

impl fmt::Display for CmdError {
    // Add some color to our error messages
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "{} -> {}", self.error_type.to_string().red(), self.msg)
    }
}