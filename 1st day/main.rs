//this code needs type annotation which i havent studied
use std::io;
//for addition,subtraction,multiplication and division from user input
fn main(){
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut num3 = String::new();
    let mut operator = String::new();
    println!("Enter first number"); 
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    println!("Enter second number"); 
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    println!("Enter third number"); 
    io::stdin().read_line(&mut num3).expect("Failed to read line");
    println!("Enter operator");
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    let num1: <f64> = num1.trim().parse().expect("Please type a number!");
    let num2: <f64> = num2.trim().parse().expect("Please type a number!");
    let num3: <f64> = num3.trim().parse().expect("Please type a number!");
    let operator: char = operator.trim().parse().expect("Please type a operator!");
    match operator {
        '+' => println!("{} + {} + {}= {}", num1, num2,num3, num1 + num2 + num3),
        '-' => println!("{} - {} - {}= {}", num1, num2,num3, num1 - num2 - num3),
        '*' => println!("{} * {} * {}= {}", num1, num2,num3, num1 * num2 * num3),
        '/' => println!("{} / {} / {}= {}", num1, num2,num3, num1 / num2 / num3),
        _ => println!("Invalid operator!"),
    }
    
}
