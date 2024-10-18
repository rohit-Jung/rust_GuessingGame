use std::{cmp::Ordering, io::stdin};
use colored::*;

use rand::Rng;

fn main() {
    println!("{}", "Number Guessing Game".bold().blue());
    println!("Guess the number");

    //Generating a random number to guess
    let random_number = rand::thread_rng().gen_range(0..10);
    println!("The random number generated is {}", random_number);

    //looping the guess 
    loop {
        //Taking the guess input from user
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");

        //parsing the guess to number to compare 
        //shadowing [guess variable] and pattern matching here  
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please guess a number");
                continue
            },
        };

        //comparing the guess with the random number
        //Ordering is a enum with three values - hence pattern matching
        match guess.cmp(&random_number){
            Ordering::Less => println!("{}", "Your guess is lower".bold().red()),
            Ordering::Equal => {
                println!("{}", "You guessed right. You win !!".bold().green());
                break;
            }
            Ordering::Greater => println!("{}", "Your guess is greater".bold().red())
        }

    }
}
