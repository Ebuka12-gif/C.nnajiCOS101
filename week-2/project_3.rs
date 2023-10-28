fn main(){
let p:f64 = 210000.00;
let r:f64 = 5.00;
let t:f64 = 3.00;

//compound interest
let c = p*(1.00-(r/100.00)).powf(t);
println!("Television value after 3 years {}", c);

}