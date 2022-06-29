fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(6, 'h');
    
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);
    
    let x1 = plus_one(5);

    println!("The value of x is: {}", x1);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // error case 
    // x + 1;
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
}


fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {}{}", value, unit_label);
}
