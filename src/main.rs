use rand::Rng;
use std::cmp::Ordering;
use std::io; // import the io library from the standard library

fn main() {
    println!("Guess the number!"); // print a line using the print macro

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // define a mutable variable using the mut keyword
        io::stdin()
            .read_line(&mut guess) // pass a reference to the guess variable using & and make it mutable because referenc are immutable by default
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
