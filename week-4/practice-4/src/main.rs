fn main() {
    let fullname = "chibudum jhon umeh";
    let department = "computer science";
    let uni = "pan-atlantic university";

    let mut school = "school of science".to_string();
    // push string
    school.push_str(" and technology");
    
    println!("my name is {}",fullname);
    //check length
    println!("the length of my fullname is {}",fullname.len());
    println!("i am a student of {} department",department );
    println!("{}",school);
    print!("{}",uni);
}
