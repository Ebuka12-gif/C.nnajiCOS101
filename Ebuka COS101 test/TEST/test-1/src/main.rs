use std::io;
fn main() {
    println!("Health diagnosis      Amount(N)       Village        discount\nAlsheimer        1,200,000         Akpabom       20%\nArrhythmia        550,000       Ngbauji        5%\nChronic kidney disease           1,500,000       Atabrikang       15%\nDiabetes          800,000       Okorobilom         10%\n450,000     Emeremen      10%");
    let mut dis = String::new();
    let mut villa = String::new();
    let mut age = String::new();
    let mut chi = String::new();
    
    println!("what illness are you suffering");
    io::stdin().read_line(&mut dis).expect("Not a valid string");
    let d = dis.trim().to_lowercase();
    
    println!("what village are you from");
    io::stdin().read_line(&mut villa).expect("Not a valid string");
    let v = villa.trim().to_lowercase();

    println!("how old are you");
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let a:f32 = age.trim().parse().expect("Not a valid number");

    println!("how many children do you have");
    io::stdin().read_line(&mut chi).expect("Not a valid string");
    let c:f32 = chi.trim().parse().expect("Not a valid number");

  if  d=="alzheimer" && v=="akpabom" && a>50.0 && c>4.0{
    let calc:f32 = 1200000.0*0.8;
     println!("you have a 20% discount and your total cost is {}",calc);
  }else{
    println!("your price is 1200000");
  }
  if d=="arrythmia" && v=="nbauji" && a>30.0 && c>4.0{
 let calc1:f32 = 550000.0*0.95;
 println!("you have a 5% discount and your total cost is {}",calc1 );
}else{
    println!("your price is 550000");
}
 
 if d=="chronic kidney disease" && v=="atabrikang" && a>40.0 && c>3.0{
 let calc2:f32 = 1500000.0*0.75;
 println!("you have a 15% discount and your total cost is {}",calc2 );
}else{
    println!("your price is 1500000");
}
 
  if d=="diabetes" && v=="okorobilom" && a>28.0 && a<45.0 && c>=2.0 && c<=4.0 {
 let calc3:f32 = 800000.0*0.1;
 println!("you have a 10% discount and your total cost is {}",calc3 );
}else{
    println!("your price is 800000");
}

if d=="arthritis" && v=="emeremen" && a>58.0 && c>=5.0{
 let calc4:f32 = 450000.0*0.1;
 println!("you have a 10% discount and your total cost is {}",calc4 );
}else{
    println!("your price is 450000");
}

}






