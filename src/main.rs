fn main() {
    println!("Please Choose Temperatur Convert");
    println!("Select 1 To Convert Fahrenheit To Celsius");
    println!("Select 2 To Convert Celsius to Fahrenheit");
    println!("Select 3 To Exit");
    
}

fn c_to_f(c:i32)->i32{
    (c*9/5)+32       
}

fn f_to_c(f:i32)->i32{
    (f-32)*5/9    
}