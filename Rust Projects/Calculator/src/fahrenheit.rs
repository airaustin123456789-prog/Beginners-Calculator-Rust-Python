// Importing the standard input-output library

use std::io;

pub fn fahrenheit_function(){

    loop{

    // Asking user for first input!

    println!("");

    println!("Please type the degrees in celsius to convert to fahrenheit! ");

    // Creating the new empty string

    let mut num1 : String = "".to_string();

    // Call upon the input-output standard library - module

    io::stdin()
    .read_line(&mut num1)
    .expect("Failed to get users first response!");

        // Checking the result if its an error or ok!

        let num1 : Result<f64,_> = num1
        .trim()
        .parse();

        if num1.is_err() {

            println!("");

            println!("Please enter a valid integer!");

            continue;

        }

        // If the value is ok we take it back!

        let num1 : f64 = num1.unwrap();

        // Making the equation

        // F = (C * 9/5) + 32

        let answer : f64 = (num1 * 9.0/5.0) + 32.0;

        println!("");

        println!("{} degrees in celsius converted to fahrenheit is {:.1} degrees.", num1, answer.round());

        println!("");

        break;

    }

}