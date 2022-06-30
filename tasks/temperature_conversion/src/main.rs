use std::io::{self, Write};

fn main() {
    
    let mut base_type = String::new();
    let mut value = String::new();

    println!("Please choose the base type:");
    println!("\t1. Celsius\n\t2. Fahrenheit");

    print!("Base type(1 or 2): ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut base_type)
        .expect("Failed to read line");
   
    print!("Please enter the value: ");
    std::io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");


    let base_type: u8 = match base_type.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    let value: f64 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => return
    };

    if base_type == 1 {
        let fahrenheit: f64 = (value * 1.8) + 32.0;
        println!("The {} degrees Celsius is {} degrees Fahrenheit", value, fahrenheit);
    } else if base_type == 2 {
        let celsius: f64 = (value - 32.0) * 5.0/9.0;
        println!("The {} degrees Fahrenheit is {} degrees Celsius", value, celsius);
    }

}
