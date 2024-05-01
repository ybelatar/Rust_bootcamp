use std::cmp::Ordering;

use ftkit::{random_number, read_number};

fn main() {
	let random_num = random_number(1..=100);

	loop {
		println!("Take a guess : ");
		let guess = read_number();

		match guess.cmp(&random_num) {
			Ordering::Less => println!("Higher\n"),
			Ordering::Greater => println!("Lower\n"),
			Ordering::Equal => {
				println!("Good Job, you guessed the right number {random_num}!!");
				break ;
			}
		}
	}
}
