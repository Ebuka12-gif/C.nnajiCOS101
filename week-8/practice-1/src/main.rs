fn main() {
    // using vec::new()
    let v : Vec<i64> = Vec::new();
    
    // println the size of vector
    println!("\nThe length of Vec::new is: {}",v.len());

    //using macro
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    //printing the size of vector
    println!("\nThe length of Vec macro is: { }",v.len());

}
