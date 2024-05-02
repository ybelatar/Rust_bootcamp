trait Contain {
	fn contained(&self, a: &i32) -> bool;
}

impl Contain for std::vec::Vec<i32> {
	fn contained(&self, a: &i32) -> bool {
		let mut b = false;
		for i in self {
			if i == a {b = true;}
		}
		b
	}
}

fn deduplicate(list: &mut Vec<i32>) {
	let mut i = 0;
	let mut occ: Vec<i32> = vec![];
	while i < list.len() {
		if !occ.contained(&list[i]) {
			occ.push(list[i]);
			i += 1;
		}
		else {list.remove(i);}
}
}


fn main() {
    let mut v = vec![1, 2, 2, 3, 2, 4, 3];
	deduplicate(&mut v);
	println!("{:#?}", v);
}
