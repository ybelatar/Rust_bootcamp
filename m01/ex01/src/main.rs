fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
	if a < b {a}
	else {b}
}



fn main() {
	let a = 2;
	let b = 3;
    println!("The minimum of {a} and {b} is {}", min(&a, &b));
}
