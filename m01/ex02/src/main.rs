trait Contain {
	fn contains(&self, a: &u8) -> bool;
}

impl Contain for std::ops::Range<u8> {
	fn contains(&self, a: &u8) -> bool {
		self.start <= *a && *a < self.end
	}
}

// It's not const I'm really sorry but I was really tempted to get into traits and methods
fn color_name(color: &[u8; 3]) -> &'static str {
	match *color {
		[0, 0, 0] => "pure black",
		[255, 255, 255] => "pure white",
		[255, 0, 0] => "pure red",
		[0, 255, 0] => "pure green",
		[0, 0, 255] => "pure blue",
		[128, 128, 128] => "perfect grey",
		[a, b, c] if a < 31 && b < 31 && c < 31 => "almost black",
		[a, b, c] if a > 128 && (0..128).contains(&b) && (0..128).contains(&c) => "redish",
		[a, b, c] if b > 128 && (0..128).contains(&a) && (0..128).contains(&c) => "greenish",
		[a, b, c] if c > 128 && (0..128).contains(&a) && (0..128).contains(&b) => "blueish",
		_ => "unknown",
	}
}

fn main() {
	let tab: [u8; 3] = [0, 0, 0];
    println!("Pure orange is the new {}!", color_name(&tab));
}




#[cfg(test)]
#[test]
fn test_lifetimes() {
    let name_of_the_best_color;

    {
        let the_best_color = [42, 42, 42];
        name_of_the_best_color = color_name(&the_best_color);
    }

    assert_eq!(name_of_the_best_color, "unknown");
}