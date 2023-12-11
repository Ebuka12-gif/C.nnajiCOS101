use std::io;
fn main() {

    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();


    let public_servants = vec![
        ("Office Administrator", vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"]), 
        ("Academic", vec!["", "Research Assistant"]), 
        ("Lawyer", vec!["Research Assistant"]), 
        ("Teacher", vec!["Research Assistant"])
        ];

    let staff_position = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    println!("welcome to public service APS level checker\nWhat is your occupation:\n");

    for (index, (public_ser, _)) in public_servants.iter().enumerate(){
        println!("For {:?} press {:?}", public_ser, index + 1);
    }

    io::stdin().read_line(&mut a).expect("Failed to read input");
    
    let occp:usize = a.trim().parse().expect("not a valid input");

    if occp == 0 || occp > public_servants.len(){
        println!("Invalid input");
        return;
    }

    let selected_job = &public_servants[occp - 1].1;

    println!("What sector of {:?} are you:\n", public_servants[occp - 1].0);

    for (index, sub_public_ser) in selected_job.iter().enumerate(){
        println!("For {:?} press {:?}", sub_public_ser, index + 1);
    }

    io::stdin().read_line(&mut c).expect("Failed to read input");
    let ant:usize = c.trim().parse().expect("not a valid input");

    
       println!("How many years of experience do you have as an Office administrator");
       io::stdin().read_line(&mut b).expect("Failed to read input");
    let ca:i32 = b.trim().parse().expect("not a valid input");

    if ca > 0 && ca < 3 {
        println!("As a/an {:?} {:?} with {:?} years of experience, you hold position {:?}",selected_job[ant],public_servants[occp - 1].0,ca,staff_position[0]);
    }

    if ca > 2 && ca < 6 {
        println!("As a/an {:?} {:?} with {:?} years of experience, you hold position {:?}",selected_job[ant],public_servants[occp - 1].0,ca,staff_position[1]);
        

    }

    if ca > 4 && ca < 9 {
        println!("As a/an {:?} {:?} with {:?} years of experience, you hold position {:?}",selected_job[ant],public_servants[occp - 1].0,ca,staff_position[2]);
        
    }

    if ca >7  && ca < 11 {
        println!("As a/an {:?} {:?} with {:?} years of experience, you hold position {:?}",selected_job[ant],public_servants[occp - 1].0,ca,staff_position[3]);
        
    }

    if ca > 10 && ca < 13 {
        println!("As a/an {:?} {:?} with {:?} years of experience, you hold position {:?}",selected_job[ant],public_servants[occp - 1].0,ca,staff_position[4]);
        
    }

    if ca > 13 {
        println!("As a/an {:?} {:?} with {:?} years of experience, you hold position {:?}",selected_job[ant],public_servants[occp - 1].0,ca,staff_position[5]);
        
    }
     
}
