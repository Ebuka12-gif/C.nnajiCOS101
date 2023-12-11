fn main() {
    // inialize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuples = {:?}"mountain_heights);

    // change 3rd and 4th elements of a mutable tuple 
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("Change tuple = {:?}", mountain_heights);
}
