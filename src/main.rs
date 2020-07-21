// a change maker app

use std::io;

#[derive(Debug)]
struct Coinage {
    quarters: i64,
    dimes: i64,
    nickels: i64,
    pennies: i64,
}

impl Coinage {
    pub fn new() -> Coinage {
        Coinage {
            quarters: 0,
            dimes: 0,
            nickels: 0,
            pennies: 0,
        }
    }
}

fn make_change(amount: f64) -> Coinage {
    println!(
        "In the make change function to create the coinage of: {}",
        amount
    );
    // println!("amount in: {}", amount);
    // since the input is expected to be a float: $16.33, multiply by the float 100.0 to get to int values of cents
    let input = (amount * 100.0).round() as i64;
    println!("input amount: {}", input);
    let mut coins = Coinage::new();

    let mut remainder = input % 25;
    coins.quarters = input / 25;

    coins.dimes = remainder / 10;
    remainder = remainder % 10;

    coins.nickels = remainder / 5;
    remainder = remainder % 5;

    coins.pennies = remainder;

    coins
}

fn main() {
    println!("Hello, please enter the amount to be made into change: ");

    let mut change = String::new();
    io::stdin()
        .read_line(&mut change)
        .expect("Sorry, I didn't catch that.");

    let change: f64 = match change.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    println!("{} will be sorted best into this coinage: ", change);

    let coinage = make_change(change);
    println!("{:?}", coinage);
}
