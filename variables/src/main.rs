fn main() {

    let mut x = 5;
    println!("The value of x is: {}", x);
    
    x = 6;
    println!("The value of x is: {}", x);
    
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    
    let y = 10;
    println!("The value of y is: {}", y);

    let y = y + 11;
    println!("The value of y is: {}", y);
    
    let spaces = "   ";
    let spaces = spaces.len();
    
    println!("The value of spaces is: {}", spaces);

    // let mut spaces2 = "   ";
    // spaces2 = spaces2.len();
    
    // mismatched types

    // expected `&str`, found `usize`rustc(E0308)
    
    // println!("The value of spaces is: {}", spaces2);

    let number: u32 = "42".parse().expect("Not a number"); //With type annotation

    println!("The value of number is: {}", number);

    // let number2 = "42".parse().expect("Not a number"); //Without type annotation

    // type annotations needed

    // consider giving `number2` a typerustc(E0282)

}
