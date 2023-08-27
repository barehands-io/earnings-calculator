use std::io;
fn main() {
    // ANSI escape code for setting text color to red
    let red = "\x1b[31m";

    // ANSI escape code for resetting text color
    let reset = "\x1b[0m";

    // Print with colored text
    println!("{}Salary Calculator !!!{}", red, reset);

    // Get input from user      
     // Prompt the number of papers delivered on the route per day
     println!("How many papers do you deliver on your route per day?");

     let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Failed to read line");
 
     let number_of_papers_per_day: i32 = input.trim().parse().expect("Please enter a valid number");
 
     println!("You deliver {} papers per day.", number_of_papers_per_day);
 
}
