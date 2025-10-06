// Importing the standard input-output library

use std::io;

pub fn addition_function() {

    loop{

    // Asking user for first input!

    println!("");

    println!("Please enter the first number to add from! ");

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

        // Creating the addition loop label!

        'addition_loop_label: loop{

        // Asking user for the second input!

            println!("");

            println!("Please enter the second number to add from! ");

              // Creating the new empty string

             let mut num2 : String = "".to_string();

             // Call upon the input-output standard library - module

             io::stdin()
             .read_line(&mut num2)
             .expect("Failed to get users first response!");

             // Checking the result if its an error or ok!

             let num2 : Result<f64,_> = num2
            .trim()
            .parse();

            if num2.is_err() {

            println!("");

            println!("Please enter a valid integer!");

            continue 'addition_loop_label

        }

             // If the value is ok we take it back!

            let num2 : f64 = num2.unwrap();


                // Making the equation

                let answer : f64 = num1 + num2;

                println!("");

                println!("{} + {} = {}.", num1, num2, answer);

                println!("");



            // Breaking the loops!


        break 'addition_loop_label;



    }



        break;
    }



}