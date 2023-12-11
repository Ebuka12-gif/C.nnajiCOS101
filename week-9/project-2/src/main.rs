use std::fs::File;
use std::io::Write;
fn main() {
    let student_name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_number = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let department = vec!["Accounting", "Economics", "Computer", "Electric", "Mechanical"];
    let level = vec!["300", "100", "200", "200", "100"];
    
    let mut file = File::create("student_management.txt").unwrap();

    writeln!(file, "STUDENT MANAGEMENT INFORMATION SYSTEM").unwrap();
    writeln!(file, "{:<20} {:<15} {:<15} {:<15}", "Name", "Matric Number", "Department", "Level");
    for i in 0..student_name.len(){
        writeln!(file, "{:<20} {:<15} {:<15} {:<15}",student_name[i], matric_number[i], department[i], level[i]);
    } 

    println!("Data written to file");

}
