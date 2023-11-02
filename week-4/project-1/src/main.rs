// Rust program to calculates the speed of a car
use std::io;

fn main() {
    let mut input1 =String::new();
    let mut input2 =String::new();

    println!("Enter the distance covered in miles:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32=input1.trim().parse().expect("Not a valid number");

    println!("Enter the number of hours:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32=input2.trim().parse().expect("Not a valid number");

    let d = a*1.6090;
    let t = b;

    let speed:f32 = d/t;
    println!("the speed of the car is {}kmph",speed);
    
}
