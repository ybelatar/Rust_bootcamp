fn add(a: &i32, b: i32) -> i32 {
	a + b
}

fn add_assign(a: &mut i32, b: i32) {
	*a = *a + b;
}

fn main() {
	let mut a = 2;
	let b = 3;
    println!("The addition of {a} and {b} is equal {}", add(&a, b));
	add_assign(&mut a, b);
    println!("The addition is also {a}");
}
