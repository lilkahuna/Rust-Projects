use std::io;

// Definition of the example_mod mod(looks for a file named after the module)
mod example_mod;
// Exports it for public use
pub use crate::example_mod::hosting;


/** IMPORTANT INFORMATION:
 * unsigned means only positive(0-255), while signed means positive or negative(-128-127)
 * a signed integer uses the first bit to store sign information. 0 for positive and vice versa.
 * 8 bits is a byte
 * a bit stores 0 or 1
 * i32 is the default integer type of rust
 * associated functions belong to the type, not the instance
 * Structs, enums, pattern matching, and impl blocks
 * Vectors are like lists
 */
fn main() 
{
    // By default, rust variables are immutable, so we must use mut to make it mutable.
    let mut guess: String = String::new(); // Creates a new empty string. :: means that new() is a function of the String type
    
    println!("Type in something");
    // & is used to reference the variable "guess"
    io::stdin().read_line(&mut guess).expect("Error occurred in reading input"); // Expect runs if the result enum gives an error varient
    
    println!("You typed {guess}");
    
    // Slice of a string
    let slice: &str = &guess[0..2];
    println!("{slice}");

    let mut v: Vec<i32> = vec![200, 100, 5000];
    
    for i in &v {
        println!("Value: {i}");
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

