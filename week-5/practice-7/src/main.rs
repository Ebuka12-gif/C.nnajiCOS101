use std::io;

fn main() {
    println!("enter a number");
    let mut input1=String::new();
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let mut num:i32=input1.trim().parse().expect("failed to input");

    while num < 10 {
        println!("inside loop number value is {}",num );
        num+=1;

    }
    println!("(outside loop number value is {}",num );
}
