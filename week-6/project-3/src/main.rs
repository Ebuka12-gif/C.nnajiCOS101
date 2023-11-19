use std::io;
fn main() {
    let mut input1 = String::new();

    println!("input a number");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let number:i32 = input1.trim().parse().expect("not a valid number");

    println!("{} multiplication table",number );

    for i in 1..13 {
        println!("{} x {} = {}", number, i, number * i );
    }

}
