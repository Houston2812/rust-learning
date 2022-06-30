use std::io::{self, Write};

fn fibonacci(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 1;
    }
    if n == 3 {
        return 2;
    } 
    return fibonacci(n-1) + fibonacci(n-2);   
}

fn main() {

    println!("Generating nth Fibonacci sequence");
    print!("Please enter the n: ");
    io::stdout().flush().unwrap();

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n) 
        .expect("Failed to read line");

    let n: u64 = match n.trim().parse() {
        Ok(num) => num,
        Err(error_) => {
            println!("Error: {error_}");
            return;
        },
    };

    let result = fibonacci(n);
    println!("The nth Fibonacci number is: {}", result);
}
