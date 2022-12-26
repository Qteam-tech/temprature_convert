use std::io;

fn main() {
    println!("**Please Input Temperature Convert Option And Press Enter**");
    println!("Input 1 To Convert Fahrenheit To Celsius");
    println!("Input 2 To Convert Celsius to Fahrenheit");
    println!("Input 3 To Exit");

    let mut option = String::new();

    io::stdin()
    .read_line(&mut option)
    .expect("Failed To Read Line");

    let option: u32 = option.trim().parse().expect("Please Input a number...");
    if option == 1 {
        println!("Please input Temprature in Fahrenheit:");
        c_to_f();
    }else if option == 2 {
        println!("Please input Temprature in Celsius:");
        f_to_c();
    }else if  option ==3 {
    }
    
}

fn c_to_f(){
    let mut temp = String::new();
        io::stdin()
        .read_line(&mut temp)
        .expect("Please Input a Number.");
        let temp:i32=temp.trim().parse().expect("Please Input a number");
    println!("Temp in Fahrenheit is : {}",(temp*9/5)+32);
    println!("");
    main();
}

fn f_to_c(){
    let mut temp = String::new();
        io::stdin()
        .read_line(&mut temp)
        .expect("Please Input a Number.");
        let temp:i32=temp.trim().parse().expect("Please Input a number");
    println!("Temp in Celsius is: {}",(temp-32)*5/9);
    println!("");
    main();
}