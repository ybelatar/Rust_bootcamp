fn collatz(start: u32) {
	let mut tmp: u32 = start;
	println!("{tmp}");
	while tmp != 1 {
		if tmp % 2 == 0 {
			tmp /= 2;
		}
		else {
			tmp = tmp * 3 + 1;
		}
		println!("{tmp}");
	}
}

fn main() {
	let start = 3;
	collatz(start);
}