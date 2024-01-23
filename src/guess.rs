use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn guessing_game() {
	let number = rand::thread_rng().gen_range(1..=100);
	println!("Guess a number between 1 and 100");

	let mut tries = 1;

	loop {
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("Error");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Guess a valid number");
				continue;
			},
		};

		match guess.cmp(&number){
			Ordering::Less => println!("Too small, guess higher"),
			Ordering::Greater => println!("Too big, guess lower"),
			Ordering::Equal => {
				println!("You guessed in {} tries", tries);
				break;
			}
		}
		tries += 1;
	}	
}