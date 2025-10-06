// Importing the standard input-output library

use std::{io, process::exit};

// Importing modules

mod multiplication;

mod division;

mod addition;

mod subtraction;

mod celsius;

mod fahrenheit;


fn main(){

    // Intro!

    println!("Welcome, to 60FpsLinuxGaming Calculator!");

    println!("");


    // Creating the main loop!

    loop{

        // Getting users operator!

        println!("Please choose an operator - 'M' for multiply, 'D' for divide, 'A' for addition, 'S' for subtraction, 'Q' for quit, 'C' to celsius, 'F' to fahrenheit! ");

        // Creating a new empty string

        let mut user_operator : String = String::new();

        // Call upon the input-output standard library - module

        io::stdin()
        .read_line(&mut user_operator)
        .expect("Failed to get users operator!");

        // Trim and uppercase users string

        let user_operator = user_operator
        .trim()
        .to_uppercase();

            // Using flow control to dictate users responses!

            if user_operator == "Q" {

                println!("");

                println!("Thank you for using my calculator!");

                exit(0);

            }

            if user_operator == "M" {

                multiplication::multiplication_function();

            }

            else if user_operator == "D" {

                division::division_function();

            }

            else if user_operator == "A" {

                addition::addition_function();

            }

            else if user_operator == "S" {

                subtraction::subtraction_function();

            }

            else if user_operator == "C" {

                celsius::celsius_function();

            }

            else if user_operator == "F" {

                fahrenheit::fahrenheit_function();

            }

            else {

                println!("");

                println!("Please enter a valid operator!");

                continue;

            }



    }

}

