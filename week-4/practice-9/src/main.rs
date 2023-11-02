fn main() {
    let A:i32 = 10;
    let B:i32 = 20;

    println!("value of A {}",A);
    println!("value of B {}",B);

    let mut res = A>B ;
    println!("A greater than B {}",res);

    res = A<B ;
    println!("A less than B {}",res); 

    res = A>=B ;
    println!("A greater than or equal to B {}",res);

    res = A<=B ;
    println!("A less than or equal to B {}",res);

    res = A==B;
    println!("A is equal to B {}",res);

    res = A!=B;
    println!("A is not equal to B {}",res);
}
