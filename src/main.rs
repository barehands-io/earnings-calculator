use colored::*;
use std::io;

fn main() {
    // Set and define constants
    const COST_OF_NEWSPAPER: f64 = 1000.00;
    const COMMISSION_IN_PERCENTAGE: f64 = 30.00;  // 30%

    const COMMISSION_IN_NAIRA: f64 = COST_OF_NEWSPAPER * (COMMISSION_IN_PERCENTAGE / 100.00);

    println!(
        "{}",
        format!(
            "Salary Calculator !!!  The cost of a newspaper is ₦{} and your commission is {}%. which is ₦{}",
            COST_OF_NEWSPAPER,
            COMMISSION_IN_PERCENTAGE,
            COMMISSION_IN_NAIRA
        ).red()
    );

    // Get input from user
    // Prompt the number of papers delivered on the route per day
    println!("How many papers do you deliver on your route per day?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number_of_papers_per_day: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid number here no alphabets please");


    println!("You deliver {} papers per day.", number_of_papers_per_day);

    // Prompt the user for the number of days the paper is delivered per week

    println!("How many days per week do you deliver papers?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number_of_days_per_week: i32 = input // shadowing
        .trim()
        .parse()
        .expect("Please enter a valid number here no alphabets please");

    if number_of_days_per_week < 1 || number_of_days_per_week > 7 {
        println!("Invalid input! There are only 7 days in a week. Please enter a number between 1 and 7.");
        // Handle the error, e.g., prompt the user again or exit the program
    }

    // calculate the total number of papers delivered per week

    let total_number_of_papers_delivered_per_week =
        number_of_papers_per_day * number_of_days_per_week;


    let weekly_sales = total_number_of_papers_delivered_per_week as f64 * COST_OF_NEWSPAPER;


    println!(
        "{}",
        format!(
            "You deliver {} papers per day and you work for only {} days per week. In total, you deliver {} papers per week.",
            number_of_papers_per_day,
            number_of_days_per_week,
            total_number_of_papers_delivered_per_week
        ).green()
    );

    // Prompt the user for the total amount of tips they receive weekly

    println!(
        "{}",
        format!(
            "How much ₦ (naira) do you receive in tips weekly  ?"
        ).yellow()
    );

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let tips_received: f32 = input
        .trim()
        .parse()
        .expect("Please enter a valid number here no alphabets please");


    println!(
        "{}",
        format!(
            "You receive ₦{} in tips weekly.",
            tips_received
        ).green()
    );

    // Calculate the total amount of money earned weekly

    let total_amount_earned_weekly = (total_number_of_papers_delivered_per_week as f64 * COMMISSION_IN_NAIRA) + (tips_received as f64);


    println!(
        "{}",
        format!(
            "if your company make ₦{} per week,  You earn ₦{} weekly. when  you deliver a total of {} papers per week and  receive {} in tips per week ",
            weekly_sales,
            total_amount_earned_weekly,
            total_number_of_papers_delivered_per_week,
            tips_received,
        ).green()
    );
}
