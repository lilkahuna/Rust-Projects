use first_project::cmd::{Command, Error, ErrorType};

#[test]
fn test_parser_for_info() {
    // Arrange
    let expected_command = Command::Info("file.txt".to_string());

    // Act
    let command_result = Command::parse_command_args(&[String::from("EXE"), String::from("info"), String::from("file.txt")]);

    // Assert
    match command_result {
        Ok(command) => {
            assert_eq!(command, expected_command);
        },
        Err(_) => {
            // If an error occurred, fail the test
            panic!("Expected Ok variant, but received Err variant");
        }
    }
}

#[test]
fn test_parser_for_error() {
    
    // Act
    let command_result = Command::parse_command_args(&[String::from("EXE"), String::from("something"), String::from("file.txt")]);
    
    // Assert
    match command_result {
        Ok(_) => {
            panic!("Expected Err variant, but received Ok variant");        
        },
        Err(er) => {
            // If an error occurred, check if it is of the expected type or pattern
            let expected_error = Error {
                msg: String::from("something is not a command."),
                error: ErrorType::InvalidArgument
            };
            assert_eq!(er, expected_error);
        }
    }
    
    
}