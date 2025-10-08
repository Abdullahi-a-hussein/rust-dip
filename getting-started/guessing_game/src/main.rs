use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    // println!("The secret number is: {secret_number}");
    loop {
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You Entered {guess}. Please Inter a valid number!");
                continue;
            },
        };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
}
}
