
/* 
Rust Implementation of MCO1
Banking and Currency Exchange Application 

The naming convention for Rust is as follows:
https://doc.rust-lang.org/1.0.0/style/style/naming/README.html

1. Functions - snake_case
2. Local variables - snake_case

*/

use std::io;   

fn validate() -> i32 {
    loop {
        let mut ans = String::new();
        io::stdin().read_line(&mut ans)
            .expect("Failed to read line");

        match ans.trim().parse::<i32>() {
            Ok(num) if num >= 1 && num <= 6 => return num,
            _ => println!("Please choose a valid option!"),
        }
    }
}

fn main_menu() {
    println!("[1] Register Account Name");
    println!("[2] Deposit Amount");
    println!("[3] Withdraw Amount");
    println!("[4] Currency Exchange");
    println!("[5] Record Exchange Rates");
    println!("[6] Show Interest Computation");

    println!();
    println!("What would you like to do?");
    let _ans = validate();
}

fn main() {
    // Display the main menu
    main_menu();

}

// TODO

// fn register_account() {

// }

// fn deposit_amount() {

// }

// fn withdraw_amount() {

// }

// fn record_exchange_rate() {

// }

// fn currency_exchange() {

// }

// fn show_interest_amount() {

// }