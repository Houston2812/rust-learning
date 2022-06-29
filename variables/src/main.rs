fn main() {
    let x = 5;
   
    println!("The value of x is: {}", x);
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    
    // difference between mut and shadowing - shadowing allows us to change variable type
    // it will cause and errror
    // because spaces is initialized as string

    // let mut spaces = "   ";
    // spaces = spaces.len();

    // to solve this issue rewrite this  as
    let _spaces = "   ";
    let _spaces = _spaces.len();
    println!("The value of x is: {}", x);
}
