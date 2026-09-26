use std::io;

fn read_input() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read input");
    let result: i32 = buffer.trim().parse().expect("Invalid number entered");
    result
}

// Moved each operation to a named function
fn add(a: i32, b: i32) {
    println!("\nResult: {}\n", a + b);
}

fn subtract(a: i32, b: i32) {
    println!("\nResult: {}\n", a - b);
}

fn multiply(a: i32, b: i32) {
    println!("\nResult: {}\n", a * b);
}

fn divide(a: f64, b: f64) {
    if b == 0.0 {
        println! ("\nNot allowed to divide by 0, returning to main menu. \n");
    }
    else {
        println!("\nResult: {}\n", a / b);
    }
}

fn main() {
    loop{
        // Fixed misused `\t`, replaced with `println!`
        println!("--- Calculator ---");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        println!("------------------\n");
        println!("Choose an option (1-5): ");

        let choice = read_input();
        // Changed inline function to `add()` calls
        if choice == 1 {
            // add
            println!("\nEnter first number:");
            let num1 = read_input();
            println!("\nEnter second number:");
            let num2 = read_input();
            add(num1, num2);
        }

        else if choice == 2 {
            // subtract
            println!("\nEnter first number:");
            let num1 = read_input();
            println!("\nEnter second number:");
            let num2 = read_input();
            subtract(num1, num2);
        }

        else if choice == 3 {
            // multiply
            println!("\nEnter first number:");
            let num1 = read_input();
            println!("\nEnter second number:");
            let num2 = read_input();
            multiply(num1, num2);
        }

        else if choice == 4 {
            // divide
            println!("\nEnter first number: \n");
            let num1 = read_input() as f64;
            println!("\nEnter second number: \n");
            let num2 = read_input() as f64;
            divide(num1, num2); // Zero check now inside `divide` function
        }

        else if choice == 5 {
            // exit
            println!("Cya math nerd o7");
            std::process::exit(0);
        }

        else {
            println!("\nInvalid option, please choose 1-5.\n");
        }
    }
}