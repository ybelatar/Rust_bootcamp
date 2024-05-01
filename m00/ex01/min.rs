fn min(a: i32, b: i32) -> i32 {
	if a < b {
		return a;
	}
	b
}

fn main() {
	let a: i32 = 45;
	let b: i32 = 12;
	println!("The minimum between {a} and {b} is : {}", min(a, b));
}