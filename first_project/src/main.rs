use std::{any::Any, borrow::Borrow, io};

struct User {
    username: String,
    password: String,
    age: u8
}

impl User {
    fn new(username: String, password: String, age: u8) -> Self {
        Self {
            username: username,
            password: password,
            age: age
        }
    }
}

impl User {
    fn is_of_age(&self) -> bool {
        if self.age >= 18 {
            true
        } else {
            false
        }
    } 
}



/** IMPORTANT INFORMATION:
 * unsigned means only positive(0-255), while signed means positive or negative(-128-127)
 * a signed integer uses the first bit to store sign information. 0 for positive and vice versa.
 * 8 bits is a byte
 * a bit stores 0 or 1
 * i32 is the default integer type of rust
 * associated functions belong to the type, not the instance
 */
fn main() 
{
    // By default, rust variables are immutable, so we must use mut to make it mutable.
    let mut guess: String = String::new(); // Creates a new empty string. :: means that new() is a function of the String type
    // Instance of User struct
    let user = User {
        username: String::from("johndoe1"),
        password: String::from("12345"),
        age: 18
    };
    println!("{}", user.username);

    println!("Type in something");
    // & is used to reference the variable "guess"
    io::stdin().read_line(&mut guess).expect("Error occurred in reading input"); // Expect runs if the result enum gives an error varient
    
    println!("Input: {guess}");
    // Slice of a string
    let slice: &str = &guess[0..2];
    println!("{}", slice);

    // Using associated function
    let new_user: User = User::new(String::from("Sigma"), String::from("Rizz"), 13); // This is like a constructor
    println!("Username: {}, Password: {}. Age: {}", new_user.username, new_user.password, new_user.age);

    // Using method syntax
    if user.is_of_age() {
        println!("User is of age");
    }

}

// Borrowing the array of i32 values
fn loop_array(arr: &[i32]) -> i32 {
    let mut counter: i32 = 0;
    for element in arr 
    {
        println!("Value of {element}; index of {counter}");
        counter += 1
    }
    // Auto return
    counter
}

// return type after ->
fn add(x: i32, y: i32) -> i32 {
    // This is an expression, so it will automatically return it's value. Expressions evaluate to a value.
    x + y
}
