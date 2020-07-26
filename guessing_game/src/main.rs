use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number: u32   = rand::thread_rng().gen_range(1,101);
    loop {
        let mut guess = String::new();
        println!("Input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed ro read line");
        println!("You guessed {}", guess);

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too small"),
            Ordering::Equal => {
                println!("That is correct");
                break;
            }
            Ordering::Greater => println!("Your guess is too big")
        }
    }
}
