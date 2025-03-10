use std::io;
use rand::Rng;

// Types, there are a few different primitivce types in rust, they are:
// i8 -> i128: signed ints of the respective bit size
// u8 -> u128: unsigned ints of the respective bit size
// f32 or f64: floats of 32 and 64 bits
// bool: Boolean
// char: char
// &str: A string literal, immutable fixed at compile time

fn main() {
    println!("Pick a number 1 - 5");

    println!("Please input your number.");

    // let is how we init a var, mut defines the var to be mutable
    // the var name is guess and we are giving it type string.
    // There are also &str that are fixed length before compile time.
    // Think about them as a c++ Array or a cstring
    let mut input_num = String::new();
    let number: i8;
    println!("Please enter a number (1-5):");

    loop {
        // Pass by reference vs pass by value. In the line below, guess must
        // be passed by value, we do this by using &mut, if we were to just give
        // give it guess, that would be by value
        io::stdin().read_line(&mut input_num).expect("Failed to read line");

        // Trim whitespace and parse the string into an integer
        let parsed_number: Result<i8, _> = input_num.trim().parse();

        match parsed_number {
            Ok(num) => {
                // Check if the number is between 1 and 5 (inclusive)
                if num >= 1 && num <= 5 {
                    number = num;  // Assign the valid number to the `number` variable
                    println!("You entered: {}", number);
                    break; // Exit the loop if valid input
                } else {
                    println!("Please enter a number between 1 and 5.");
                    input_num.clear();  // Clear the inputNum to prepare for a new input
                }
            },
            Err(_) => {
                println!("Invalid input, please enter a valid number.");
                input_num.clear();  // Clear the inputNum to prepare for a new input
            },
        }
    }

    let guess_count: i8 = check_if_guess_is_correct(number);
    println!("It took {} attempts to guess correctly.", guess_count);
}

fn check_if_guess_is_correct(input_num: i8) -> i8{
    let mut guess_count: i8 = 0;
    let mut rng = rand::thread_rng(); // Create a random number generator
    loop{
        guess_count += 1;
        let random_number: i8 = rng.gen_range(1..=5);
        println!("I'm guessing your number is {}", random_number);
        if input_num == random_number {
            return guess_count;
        }
    }
}