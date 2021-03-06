
use rand::Rng;
use std::{cmp::Ordering, io};

fn basics() {
    let secret_number = rand::thread_rng().gen_range(0, 101);

    println!("Guess a number!");

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please insert a number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(it) => it,
            Err(_) => continue
        };

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}