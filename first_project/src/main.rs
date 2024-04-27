use std::io;

/** IMPORTANT INFORMATION:
 * unsigned means only positive(0-255), while signed means positive or negative(-128-127)
 * a signed integer uses the first bit to store sign information. 0 for positive and vice versa.
 * 8 bits is a byte
 * a bit stores 0 or 1
 * i32 is the default integer type of rust
 */
fn main() 
{
    // By default, rust variables are immutable, so we must use mut to make it mutable.
    let mut guess: String = String::new(); // Creates a new empty string. :: means that new() is a function of the String type
    
    println!("Type in something");
    // & is used to reference the variable "guess"
    io::stdin().read_line(&mut guess).expect("Error occurred in reading input"); // Expect runs if the result enum gives an error varient
    
    println!("Input: {guess}");

    // i32 is the type of the array, and the 5 is the amount of elements
    let arr: [i32; 5] = [200, 23, 6, 7, 8];

    let mut counter: i32 = 0;
    for element in arr 
    {
        println!("Value of {element}; index of {counter}");
        counter += 1
    }

    let sum: i32 = add(12, 12);
    println!("Sum is {sum}")
}

// return type after ->
fn add(x: i32, y: i32) -> i32
{
    // This is an expression, so it will automatically return it's value. Expressions evaluate to a value.
    x + y
}
