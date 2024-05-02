// fn max(a: usize, b: usize) -> usize {
// 	if a > b {a}
// 	else {b}
// }

fn to_num(a: u8) -> u8 {
	a - b'0'
}

fn to_ascii(a: u8) -> u8 {
	a + b'0'
}

fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
	let mut res: Vec<u8> = vec![];
	assert!(!a.is_empty() && !b.is_empty());
	let mut carry = 0;
	for i in (0..a.len()).rev() {
		assert!(a[i].is_ascii_digit() && b[i].is_ascii_digit());
		println!("a {} ,b {}", a[i], b[i]);
		match to_num(a[i]) + to_num(b[i]) + carry {
			num if num > 9 => {
				carry = 1;
				res.push(to_ascii(num - 10));
			}
			num => {
				carry = 0;
				res.push(to_ascii(num));
			}
		}
	}
	res.reverse();
	while res[0] == b'0' {res.remove(0);}
	res
}




fn main() {
    println!("{:?}", big_add(b"125", b"125"));
}


#[cfg(test)]
#[test]
fn test1() {assert_eq!(big_add(b"2", b"4"), b"6");}
#[test]
fn test2() {assert_eq!(big_add(b"0010", b"0200"), b"210");}
#[test]
fn test3() {assert_eq!(big_add(b"125", b"125"), b"250");}
#[test]
#[should_panic]
fn test_panic1() {assert_eq!(big_add(b"a2", b"40"), b"6");}
#[test]
#[should_panic]
fn test_panic2() {assert_eq!(big_add(b"", b"40"), b"6");}