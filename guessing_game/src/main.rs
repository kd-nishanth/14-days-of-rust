use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number?");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut tries_left: u32 = 5;
    let tries_min: u32 = 0;
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read a line");
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };
        let guess: u32 = guess.trim().parse().expect("Please Input a number");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        tries_left -= 1;
        println!("You have {tries_left} tries left");

        match tries_left.cmp(&tries_min) {
            Ordering::Equal => {
                println!("You lose!");
                break;
            }
            _ => continue,
        }
    }
}
