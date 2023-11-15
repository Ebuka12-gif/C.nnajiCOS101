use std::io;
fn main() {
    let mut input1=String::new();
    let mut input2=String::new();
    let mut input3=String::new();
   
    println!("\nEnter the first value of the quadractic equation (ax^2):");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("\nEnter the second value of the quadractic equation (bx):");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("\nEnter the third value of the quadractic equation (c):");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let y= b.powf(2.0);
    let s=4.0*a*c;
    let d=y-s;
    let j=b*-1.0;
    let k=d.sqrt();
    let t=j+k;
    let m=j-k;
    let q=a*2.0;
    let l=t/q;
    let z=m/q;
   println!("the roots of the quadractic equation are {}, {}",l,z );
}   

