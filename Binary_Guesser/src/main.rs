use std::io;

fn main() {
    println!("Pick a number 1 - 50");

    println!("Please input your number.");

    let mut input_num = String::new();
    let number: i8;

    // This loop establishes the number chose between 1-50, the binary guesser will then guess your number
    loop {
        io::stdin().read_line(&mut input_num).expect("Failed to read line");

        // Trim whitespace and parse the string into an integer
        let parsed_number: Result<i8, _> = input_num.trim().parse();

        match parsed_number {
            Ok(num) => {
                // Check if the number is between 1 and 50 (inclusive)
                if num >= 1 && num <= 50 {
                    number = num;  // Assign the valid number to the `number` variable
                    println!("You entered: {}", number);
                    break; // Exit the loop if valid input
                } else {
                    println!("Please enter a number between 1 and 50.");
                    input_num.clear();  // Clear the inputNum to prepare for a new input
                }
            },
            Err(_) => {
                println!("Invalid input, please enter a valid number.");
                input_num.clear();  // Clear the inputNum to prepare for a new input
            },
        }
    }

    binary_search(number);
}

fn binary_search(input_num: i8){
    let mut step_count: i8 = 0;
    let mut upper: i8 = 50;
    let mut lower: i8 = 1;
    let mut guess_num: i8;
    loop{
        step_count += 1;
        guess_num = lower + ((upper - lower) / 2); // Should trunc the decimal
        if guess_num == input_num{
            println!("Your number is {}! It took {} guesses.", guess_num, step_count);
            break;
        }
        println!("I'm going to guess {}", guess_num);
        let h_or_l: bool = high_or_low();
        if h_or_l{
            lower = guess_num;
        } else{
            upper = guess_num;
        }
    }
}

// return true if the number is higher, false if the number is lower
fn high_or_low() -> bool{
    // Is your number higher or lower?
    println!("Is your number higher or lower? Enter `h` for higher, and `l` for lower.");
    loop{
        let mut high_low = String::new();
        io::stdin().read_line(&mut high_low).expect("Failed to read line");
        let input_string = high_low.trim();

        if input_string == "h"{
            println!("Higher!");
            return true;
        } else if input_string == "l"{
            println!("Lower!");
            return false;
        } else {
            println!("Invalid input, please input either `h` or `l`");
        }
    }
}