use std::io::{self};

fn calculate(choice: u32, num1: f64, num2: f64) -> Result<(f64, &'static str), String> {
    match choice {
        1 => Ok((num1 + num2, "+")),
        2 => Ok((num1 - num2, "-")),
        3 => Ok((num1 * num2, "*")),
        4 => {
            if num2 == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok((num1 / num2, "/"))
            }
        }
        _ => unreachable!(),
    }
}

fn get_number(prompt: &str) -> Result<f64, String> {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Invalid input! Please enter a valid number".to_string())?;

    input
        .trim()
        .parse()
        .map_err(|_| "Invalid input! Please eneter a valid number.".to_string())
}

fn get_valid_number(prompt: &str) -> f64 {
    loop {
        match get_number(prompt) {
            Ok(num) => return num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}
fn main() {
    loop {
        println!(
            "
            Rust CLI calculator
            Available operations:
            1. Addition (+)
            2. Subtraction (-)
            3. Multiplication (*)
            4. Division (/)
            5. Exit
            "
        );

        println!("\nPlease select an operation (1-5): ");
        let mut choice = String::new();
        // io::stdin()
        //     .read_line(&mut choice)
        //     .expect("Failed to read line!");
        match io::stdin().read_line(&mut choice) {
            Ok(_) => (),
            Err(_) => {
                println!("Error reading input! Please try again.");
                continue;
            }
        }

        let choice: u32 = match choice.trim().parse() {
            Ok(num) if (1..=5).contains(&num) => num,
            Ok(_) => {
                println!("Invalid operation! Please select a number between 1 and 5.");
                continue;
            }
            Err(_) => {
                println!("Invalid input! Please enter a number between 1 and 5.");
                continue;
            }
        };

        if choice == 5 {
            println!("Thank you for using the calculator. Exiting the program!");
            break;
        }

        // if choice < 1 || choice > 5 {
        //     println!("Invalid operation! Please select a number between 1 and 5.!");
        //     continue;
        // }

        // // Get first number
        // println!("Enter first number:");
        // let mut num1 = String::new();
        // io::stdin()
        //     .read_line(&mut num1)
        //     .expect("Failed to read line");

        // let num1: f64 = match num1.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => {
        //         println!("Invalid input! Please enter a valid number.");
        //         continue;
        //     }
        // };

        // Get second number
        // println!("Enter second number:");
        // let mut num2 = String::new();
        // io::stdin()
        //     .read_line(&mut num2)
        //     .expect("Failed to read line");

        // let num2: f64 = match num2.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => {
        //         println!("Invalid input! Please enter a valid number.");
        //         continue;
        //     }
        // };

        // // Get first number
        let num1 = get_valid_number("Enter first number: ");
        // // Get second number
        let num2 = get_valid_number("Enter second number: ");

        // // Get first number
        // let num1 = match get_number("Enter first number: ") {
        //     Ok(num) => num,
        //     Err(e) => {
        //         println!("{}", e);
        //         continue;
        //     }
        // };

        // // Get second number
        // let num2 = match get_number("Enter second number: ") {
        //     Ok(num) => num,
        //     Err(e) => {
        //         println!("{}", e);
        //         continue;
        //     }
        // };

        //Calculation based on choice. Improving readability and reusability
        match calculate(choice, num1, num2) {
            Ok((result, operator)) => println!("{} {} {} = {}", num1, operator, num2, result),
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }

        //Perform calculation and get operator
        // let (result, operator) = match choice {
        //     1 => (num1 + num2, "+"),
        //     2 => (num1 - num2, "-"),
        //     3 => (num1 * num2, "*"),
        //     4 => {
        //         if num2 == 0.0 {
        //             println!("Error: Division by zero!");
        //             continue;
        //         }
        //         (num1 / num2, "/")
        //     }
        //     _ => unreachable!(),
        // };

        // // Perform calculation based on choice
        // let result = match choice {
        //     1 => num1 + num2,
        //     2 => num1 - num2,
        //     3 => num1 * num2,
        //     4 => {
        //         if num2 == 0.0 {
        //             println!("Error: Division by zero!");
        //             continue;
        //         }
        //         num1 / num2
        //     }
        //     _ => unreachable!(),
        // };

        // // Display result
        // let operator = match choice {
        //     1 => "+",
        //     2 => "-",
        //     3 => "*",
        //     4 => "/",
        // _ => unreachable!(),
        // };

        // println!("{} {} {} = {}", num1, operator, num2, result);
    }
}
