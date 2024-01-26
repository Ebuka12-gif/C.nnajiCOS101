use std::fs::File;
use std::io::Write;
use std::io;
fn main() {
    let mut user = String::new();
    let mut pass = String::new();
    println!("Please input your user name");
    io::stdin().read_line(&mut user).expect("Failed to read input");
    let name = user.trim().to_lowercase();
    
      if name.len()<3 || name.len()>8{ 
    println!("username not valid");
    return }

    println!("Please input your password");
    io::stdin().read_line(&mut pass).expect("Failed to read input");
    let _password = pass.trim().to_lowercase();   
   
      let x = 15000000 - 5500000;
    let y = 25000000 - 8000000;
    let f = 18000000 - 10000000;
    let g = 32000000 - 4000000;
    let h = 8000000 - 1500000;
    let t = 37000000 - 11000000;
    let z = 34000000 - 9000000;
    let j = 30000000 - 12000000;
    
    let xl:i64 = x*100/15000000;
    let yl:i64 = y*100/25000000;
    let fl:i64 = f*100/18000000;
    let gl:i64 = g*100/32000000;
    let hl:i64 = h*100/8000000;
    let tl:i64 = t*100/37000000;
    let zl:i64 = z*100/34000000;
    let jl:i64 = j*100/30000000;


     let company = vec!["Cadbury Nigeria Plc", "Champion Breweries Plc", "Dangote Sugar Refinery Plc", "Flour Mills Nigeria Plc", "Nestle Nigeria Plc", "Unilever Nigeria Plc", "Honeywell Nigeria Plc", "Nigerian breweries Plc"]; 
    let date = vec!["1965", "1974", "1970", "1960", "1961", "1923", "1906", "1946"]; 
    let assets = vec!["15000000", "25000000", "18000000", "32000000", "8000000", "37000000", "34000000", "30000000"];
    let liabilities =vec!["5500000", "8000000", "10000000", "4000000", "1500000", "11000000", "9000000", "12000000"];          
    let leverage_percent: Vec<&i64> =vec![&xl, &yl, &fl, &gl, &hl, &tl, &zl, &jl];

    
  let mut file = File::create("Nigerian Company.txt").unwrap();

    writeln!(file, "These are the Nigerian companies and leverages over the years ").unwrap();
    writeln!(file, "{:<20}    {:<15} {:<15} {:<15} {:<15}", "company", "date", "assets", "liabilities", "leverage percent");
    for i in 0..assets.len(){
        writeln!(file, "{:<20}    {:<15} {:<15} {:<15} {:<15}",company[i], date[i], assets[i], liabilities[i], leverage_percent[i]);
    } 
    
    println!("Data written to file");

}

      

     






