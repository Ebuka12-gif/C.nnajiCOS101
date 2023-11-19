use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input4= String::new();
    let mut input5 = String::new();
    let mut input7 = String::new();
    let mut input3= String::new();
    let mut input2= String::new();
    let mut input6 = String::new();


    println!("Welcome to the Pan-Atlantic student council voting system\nWhat is your name");
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let name:String = input1.trim().parse().expect("input not an integer");

    println!("What state are you from");
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let state:String = input2.trim().parse().expect("input not an integer");
  
    println!("Please input your E-mail.");
    io::stdin().read_line(&mut input3).expect("failed to read input");
    let email:String = input3.trim().parse().expect("input not an integer");

    println!("What department are you in ?");
    io::stdin().read_line(&mut input6).expect("failed to read input"); 
    let department:String = input6.trim().parse().expect("input not an integer");

    println!("What level are you in currently, ;\nFor 100 level, input 1\nFor 200 level, input 2\nFor 300 level, input 3\nFor 400 level, input 4");
    io::stdin().read_line(&mut input5).expect("failed to read input");
    let class:i32 = input5.trim().parse().expect("input not an integer");

    println!("Are you a current class rep. if yes type 1.\nif no type 0");
    io::stdin().read_line(&mut input4).expect("failed to read input");
    let rep:i32 = input4.trim().parse().expect("Not a valid number");

    println!("Please input your current CGPA");
    io::stdin().read_line(&mut input7).expect("failed to read input");
    let cgpa:f32 = input7.trim().parse().expect("Not a valid number");

    if class >= 2 && class <= 4 && cgpa > 4.0 && cgpa <= 5.0 && rep == 1{
        println!("{}", name );
        println!("{}", state );
        println!("{}", email );
        println!("Class representative of {}", department );
        println!("{}",class );
        println!("{} You are eligible to vote",name);
    }else{
        println!("You are not eligible to vote");
    }  


}
