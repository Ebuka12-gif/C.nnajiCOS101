use std::fs::File;
use std::io::Write;
fn main() {
let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
let stout = vec!["Legend", "Turbo King", "Williams", "", "", ""]; 
let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz", "", ""];
           
           let mut file = File::create("Nigerian Brewweries Plc.txt").unwrap();

    writeln!(file, "Nigerians number one choice breweries").unwrap();
    writeln!(file, "{:<20} {:<15} {:<15}", "Lager", "Stout", "Non-Alcoholic");
    for i in 0..lager.len(){
        writeln!(file, "{:<20} {:<15} {:<15}",lager[i], stout[i], non_alcoholic[i]);
    } 

    println!("Data written to file");

}

    

