fn main() {
    let fullname = " pan-atlantic university ";
    println!();
    println!("name: {}", fullname);
    println!();
    println!("Before trim");
    println!("length is {}",fullname.len());
    println!();
    println!("after trim");
    println!("length is {}",fullname.trim().len());
}
