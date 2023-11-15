

use std::io;

fn main() {
    println!("Enter your experience (Experienced/Inexperienced): ");
    let mut employee_experience = String::new();
    io::stdin().read_line(&mut employee_experience).expect("Failed to read input");
    let employee_experience = employee_experience.trim();

    println!("Ënter your age: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Failed to read input");

    if age >= 40 && employee_experience == "Experienced" {
        
        println!("Incentive = N1,560,000");
    }
    
    else if age >= 30 && age < 40 && employee_experience == "Experienced" { 
            println!("Incentive = N1,480,000");

    }
    else if age < 30 && employee_experience == "Experienced" {
            println!("Incentive = N1,300,000 per month");
    }
    else if employee_experience == "Inexperienced" {
            println!("Incentive = N100,000");
    }
}
