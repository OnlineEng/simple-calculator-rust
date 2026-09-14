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
            println!("Enter first number:");
            let num1 = read_input();
            println!("Enter second number:");
            let num2 = read_input();
            println!("Result: {}\n", num1 + num2);
        }

        else if choice == 2 {
            // subtract
            println!("Enter first number:");
            let num1 = read_input();
            println!("Enter second number:");
            let num2 = read_input();
            println!("Result: {}\n", num1 - num2);
        }

        else if choice == 3 {
            // multiply
            println!("Enter first number:");
            let num1 = read_input();
            println!("Enter second number:");
            let num2 = read_input();
            println!("Result: {}\n", num1 * num2);
        }

        else if choice == 4 {
            // divide
            println!("Enter first number:");
            let num1 = read_input();
            println!("Enter second number:");
            let num2 = read_input();
            println!("Result: {}\n", num1 / num2);
        }

        else if choice == 5 {
            // exit
            println!("Cya math nerd");
            std::process::exit(0);
        }


    }
}