use std::io;

fn main() {
    println!("Please Input Temperature Convert Option And Press Enter.");
    println!("Input 1 To Convert Fahrenheit To Celsius");
    println!("Input 2 To Convert Celsius to Fahrenheit");
    println!("Input 3 To Exit");

    let mut option = String::new();

    io::stdin()
    .read_line(&mut option)
    .expect("Failed To Read Line");

}

fn c_to_f(c:i32)->i32{
    (c*9/5)+32       
}

fn f_to_c(f:i32)->i32{
    (f-32)*5/9    
}