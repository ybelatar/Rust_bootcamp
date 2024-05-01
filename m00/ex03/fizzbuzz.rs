fn main() {
	for i in 1..=100 {
		if i%3==0 && i%5==0 {println!("fizzbuzz");}
		else if i%3==0 {println!("fizz");}
		else if i%5==0 {println!("buzz");}
		else if i%11==3 {println!("FIZZ");}
		else if i%11==5 {println!("BUZZ");}
		else {println!("{i}");}
	}
}