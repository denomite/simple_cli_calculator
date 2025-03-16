use std::io::{self};

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
fn main() {
    println!("Rust CLI calculator");

    loop {
        println!("\nAvailable operations: ");
        println!("1. Addition (+)");
        println!("2. Subtraction (-)");
        println!("3. Multiplication (*)");
        println!("4. Division (/)");
        println!("5. Exit");

        println!("\nPlease select an operation (1-5): ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line!");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number between 1 and 5.");
                continue;
            }
        };

        // Exit choice
        if choice == 5 {
            println!("Thank you for using the calculator. Exiting the program!");
            break;
        }

        if choice < 1 || choice > 5 {
            println!("Invalid operation! Please select a number between 1 and 5.!");
            continue;
        }

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

        // Get first number
        let num1 = match get_number("Enter first number: ") {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        // Get second number
        let num2 = match get_number("Enter second number: ") {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        // Perform calculation based on choice
        let result = match choice {
            1 => num1 + num2,
            2 => num1 - num2,
            3 => num1 * num2,
            4 => {
                if num2 == 0.0 {
                    println!("Error: Division by zero!");
                    continue;
                }
                num1 / num2
            }
            _ => unreachable!(),
        };

        // Display result
        let operator = match choice {
            1 => "+",
            2 => "-",
            3 => "*",
            4 => "/",
            _ => unreachable!(),
        };

        println!("{} {} {} = {}", num1, operator, num2, result);
    }
}
