use std::io;

fn read_input() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read input");
    let result: i32 = buffer.trim().parse().expect("Invalid number entered");
    result
}

fn main() {
    loop{
        println!("--- Calculator --- \t
1. Add \t
2. Subtract \t
3. Multiply \t
4. Divide \t
5. Exit \t
-----------------\n
Choose an option (1-5): ");

        let choice = read_input();
        if choice == 1 {
            // add
            println!("\nEnter first number:");
            let num1 = read_input();
            println!("\nEnter second number:");
            let num2 = read_input();
            println!("\nResult: {}\n", num1 + num2);
        }

        else if choice == 2 {
            // subtract
            println!("\nEnter first number:");
            let num1 = read_input();
            println!("\nEnter second number:");
            let num2 = read_input();
            println!("\nResult: {}\n", num1 - num2);
        }

        else if choice == 3 {
            // multiply
            println!("\nEnter first number:");
            let num1 = read_input();
            println!("\nEnter second number:");
            let num2 = read_input();
            println!("\nResult: {}\n", num1 * num2);
        }

        else if choice == 4 {
            // divide
            // fix for decimal division -- converts to f64 AFTER, 0.0 change needed
            println!("\nEnter first number: \n");
            let num1 = read_input() as f64;
            println!("\nEnter second number: \n");
            let num2 = read_input() as f64;
            if num2 != 0.0 {
                println!("\nResult: {}\n", num1 / num2);
            }
            else {
                println ! ("\nNot allowed to divide by 0, returning to main menu. \n");
            }
        }

        else if choice == 5 {
            // exit
            println!("Cya math nerd o7");
            std::process::exit(0);
        }
    }
}