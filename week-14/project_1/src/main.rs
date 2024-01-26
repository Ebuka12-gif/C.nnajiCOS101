use std::io::Read;
use std::io;
fn main() {
let mut input1 = String::new();
    println!("Please input one of the following numbers (1-5)\n1 Administrator\n2 Project Manager\n3 Employee\n4 Customer\n5 Vendor");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let input1:i64 = input1.trim().parse().expect("not a valid number");
  if input1 == 1 {
  println!("You have chosen Administrator");
    open_admin()
  }else if input1 == 2 {
    println!("You have chosen Project Manager");
    open_project()
  }else if input1 == 3 {
    println!("You have chosen Employee");
    open_staff()
  }else if input1 == 4 {
   println!("You have chosen Customer");
    open_customer()
  }else if input1 == 5 {
    println!("You have chosen Vendor");
    open_dataplan()
  }else { println!("Invalid input")
}

}

fn open_customer () {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn open_staff () {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn open_project () {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn open_admin () {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn open_dataplan () {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
