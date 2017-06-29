extern crate rand;

use std::io;
use rand::Rng;

struct Status {
	score: u32,
	word: String,
	guesses: Vec<char>,
	hidden: Vec<String>,
	failed: u8
}

fn main() {

	println!("Welcome to hangman");

	let mut status = Status{
		score: 0,
		word: get_random_word(),
		guesses: Vec::new(),
		hidden: Vec::new(),
		failed: 0
	};

	play(&mut status);

}

fn play(status: &mut Status) -> () {

	println!("\nYour score is: {}", status.score);
	println!("\n");

	status.hidden = Vec::new();
	status.guesses = Vec::new();
	status.failed = 0;
	status.word = get_random_word();

	//  Initialize the array of hidden chars
	for _i in 0..status.word.len() {
		status.hidden.push(String::from("_"));
	}
	println!("Here is your word:\n");
	println!("{}", status.hidden.join(" "));
	println!("\n");

	loop {

		println!("Input your guess: ");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");

		println!("\n");

		let mut i = 0;
		let mut found: bool = false;
		for letter in status.word.chars() {
			let c = guess.chars().next();
			if c == Some(letter) {
				found = true;
				status.hidden[i] = letter.to_string();
				status.guesses.push(letter);
			}
			i += 1;
		}

		if found {
			println!("\"{}\" is in the word!\n", guess.trim());
			draw_man(&status);
		} else {
			status.failed += 1;
			println!("\"{}\" is NOT in the word!\n", guess.trim());
			draw_man(&status);
			if status.failed == 6 {
				println!("Oh no! You lost!");
				if ask_for_another_game(&status) {
					play(status);
					break;
				}
				break;
			}
		}

		println!("{}", status.hidden.join(" "));
		println!("\n");

		if status.guesses.len() == status.hidden.len() {
			println!("You win!\n");
			status.score += 1;
			if ask_for_another_game(&status) {
				play(status);
				break;
			}
			break;
		}
	}

}

fn ask_for_another_game(status: &Status) -> bool {
	let mut playagain = String::new();
	println!("Want to play again? y/n\n");
	io::stdin().read_line(&mut playagain)
		.expect("Failed to read line");
	if playagain.trim() == "y" {
		return true;
	}
	else if playagain.trim() == "n" {
		return false;
	}
	else {
		println!("Please answer \"y\" or \"n\"");
		return ask_for_another_game(&status);
	}
}

fn draw_man(status: &Status) -> () {
	println!("\n#################################\n");
	println!("|----|");
	if status.failed == 6 {
		println!("|  (xx)");
		println!("|   /\\");
		println!("|   |");
		println!("|   /\\");
	}
	if status.failed == 5 {
		println!("|    o");
		println!("|   /\\");
		println!("|   |");
		println!("|   /");
	}
	if status.failed == 4 {
		println!("|    o");
		println!("|   /\\");
		println!("|   |");
		println!("|");
	}
	if status.failed == 3 {
		println!("|    o");
		println!("|   /\\");
		println!("|");
		println!("|");
	}
	if status.failed == 2 {
		println!("|    o");
		println!("|   /");
		println!("|");
		println!("|");
	}
	if status.failed == 1 {
		println!("|    o");
		println!("|");
		println!("|");
		println!("|");
	}
	if status.failed == 0 {
		println!("|");
		println!("|");
		println!("|");
		println!("|");
	}

	println!("|_______");
	println!("#################################\n");
}

fn get_random_word() -> String {

	let words = ["test", "test2"];

	let random_number = rand::thread_rng().gen_range(0, words.len());

	String::from(words[random_number])
}

