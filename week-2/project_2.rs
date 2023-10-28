fn main(){
let t:f64 = 450000.00;
let m:f64 = 1500000.00;
let h:f64 = 750000.00;
let d:f64 = 2850000.00;
let a:f64 =250000.00;

//sum
let s = t+m+h+d+a;
println! ("Sum is {}", s);

//average
let x = (2.0*t)+(3.0*h)+(3.0*d)+m+a;
let v = x/5.0;
println!("Average is {}", v);



}