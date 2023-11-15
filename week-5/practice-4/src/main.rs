use std::io;

fn main() {
   let mut input1=String::new();
   let mut input2=String::new();

   println!("Enter your name");
   io::stdin().read_line(&mut input1).expect("Not a valid string ");
 
   println!("Enter your age");
   io::stdin().read_line(&mut input2).expect("Not a valid string ");
   let age:f32 = input2.trim().parse().expect("Not a valid number");

   if age >=18.0{
    println!("Welcome to the party {}!",input1);
   } else {
    println!("Oops you are not of age to enter the party {}",input1 );

   } 
}

