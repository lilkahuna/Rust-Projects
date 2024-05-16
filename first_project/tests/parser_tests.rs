use first_project::cmd::{Command, Error};

#[test]
fn test_parser() {
    // Arrange
    let expected_command = Command::Info("file.txt".to_string());

    // Act
    let command_result = Command::parse_command_args(&[String::from("EXE") ,String::from("info"), String::from("file.txt")]);

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