use std::io;
fn main() {
    let mut input1=String::new();
    let mut input2=String::new();
    let mut input3=String::new();

    println!("What is your name?");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let name=input1.trim().to_lowercase(); 

    println!("Please input the number of papers published");
    io::stdin().read_line( &mut input2).expect("not a valid number");
    let number:f32=input2.trim().parse().expect("not a valid number");

    println!("Please input your queue number");
    io::stdin().read_line( &mut input3).expect("not a valid number");
    let no_of_ppl:f32=input3.trim().parse().expect("not a valid number");

    if no_of_ppl <= 150.0 {

        if number >= 3.0 && number <= 5.0 {
            println!("{} your incentive is $500000",name);
        } else if number > 5.0 && number <= 10.0 {
            println!("{} your incentive is $800000",name);
        } else if number >= 10.0 {
            println!("{} your incentive is $1000000",name);
        } else if number < 3.0 {
            println!("{} your incentive is $100000",name);
        } 
     } else {
        println!("Not eligible");
    }
}
