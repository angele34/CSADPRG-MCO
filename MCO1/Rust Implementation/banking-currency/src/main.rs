/* 
Rust Implementation of MCO1
Banking and Currency Exchange Application 
Author: Angel Arwen E. Reyes
Group: 7

---

The naming convention for Rust is as follows:
https://doc.rust-lang.org/1.0.0/style/style/naming/README.html

1. Functions - snake_case
2. Local variables - snake_case

Sources used:
    Fix for reading input before the print statement
    1. https://doc.rust-lang.org/std/macro.print.html
    Data types
    2. https://doc.rust-lang.org/book/ch03-02-data-types.html

*/

use std::io::{self, Write};

struct Account {
    account_name : String,
    current_balance : f64,
}

fn validate() -> i32 {
    loop {
        let mut ans = String::new();
        io::stdin()
            .read_line(&mut ans)
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
    let ans = validate();

    match ans {
        1 => { register_account(); },
        2 => deposit_amount(),
        3 => withdraw_amount(),
        4 => currency_exchange(),
        5 => record_exchange_rate(),
        6 => show_interest_amount(),
        _ => unreachable!(),
    }
}

fn register_account() -> Account {
    println!("Register Account Name");
    // Ask user for their account name
    print!("Account Name: ");
    io::stdout().flush().unwrap();

    let mut ans = String::new();
    io::stdin()
        .read_line(&mut ans)
        .expect("Failed to read line");

    let name = ans.trim().to_string();

    let account = Account {
        account_name: name,
        current_balance: 0.0,
    };

    // Go back to main menu
    println!("Back to Main Menu (Y/N)");
    account
}

fn deposit_amount() {
    println!("Deposit amount");
}

fn withdraw_amount() {
    println!("Withdraw amount");
}

fn record_exchange_rate() {
    println!("Record exchange rate");
}

fn currency_exchange() {
    println!("Currency exchange");
}

fn show_interest_amount() {
    println!("Show interest amount");
}

fn main() {
    // Display the main menu
    main_menu();
}