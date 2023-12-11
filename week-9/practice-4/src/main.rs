use std::fs::OpenOptions;
use Std::io::Write;
fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file")'
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
         .as_bytes()).expect("write failed")
    println!("file append success");
}
