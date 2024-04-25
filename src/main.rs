use std::io;

/** IMPORTANT INFORMATION:
 * unsigned means only positive(0-255), while signed means positive or negative(-128-127)
 * a signed integer uses the first bit to store sign information. 0 for positive and vice versa.
 * 8 bits is a byte
 * a bit stores 0 or 1
 */
fn main() 
{
    // By default, rust variables are immutable, so we must use mut to make it mutable.
    let mut guess: String = String::new(); // Creates a new empty string. :: means that new() is a function of the String type

    // & is used to reference the variable "guess"
    io::stdin().read_line(&mut guess).expect("Error occurred in reading input"); // Expect runs if the result enum gives an error varient

    println!("Input: {guess}");
}

