fn max(a: usize, b: usize) -> usize {
	if a > b {a}
	else {b}
}

fn to_num(a: u8) -> u8 {
	a - b'0'
}

fn to_ascii(a: u8) -> u8 {
	a + b'0'
}

fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
	assert!(!a.is_empty() && !b.is_empty());

	let mut res: Vec<u8> = Vec::new();
	let mut carry = 0;

	let maximum = max(a.len(), b.len());
	for i in 1..=maximum {
		let abis = if a.len() >= i {a[a.len() - i]} else {b'0'};
		let bbis = if b.len() >= i {b[b.len() - i]} else {b'0'};
		assert!(abis.is_ascii_digit() && bbis.is_ascii_digit());
		println!("a {} ,b {}", abis, bbis);
		match to_num(abis) + to_num(bbis) + carry {
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
	if carry == 1 {res.push(b'1');}
	res.reverse();
	while res[0] == b'0' && res.len() > 1 {res.remove(0);}
	res
}




fn main() {
    println!("{:?}", big_add(b"0", b"0"));
}


#[cfg(test)]
#[test]
fn zero_plus_zero() {
    assert_eq!(big_add(b"0", b"0"), b"0");
}

#[cfg(test)]
#[test]
#[should_panic]
fn empty_string() {
    assert_eq!(big_add(b"", b"123"), b"");
}

#[cfg(test)]
#[should_panic]
#[test]
fn non_digits() {
    assert_eq!(big_add(b"abc", b"123"), b"");
}

#[cfg(test)]
#[test]
fn basic_add() {
    assert_eq!(big_add(b"1", b"1"), b"2");
    assert_eq!(big_add(b"10", b"1"), b"11");
    assert_eq!(big_add(b"100", b"23"), b"123");
}

#[cfg(test)]
#[test]
fn middle_carry() {
    assert_eq!(big_add(b"1", b"19"), b"20");
    assert_eq!(big_add(b"5", b"18"), b"23");
}

#[cfg(test)]
#[test]
fn end_carry() {
    assert_eq!(big_add(b"9", b"1"), b"10");
    assert_eq!(big_add(b"9", b"3"), b"12");
    assert_eq!(big_add(b"900", b"100"), b"1000");
}

#[cfg(test)]
#[test]
fn full_carry() {
    assert_eq!(big_add(b"999", b"1"), b"1000");
}

#[cfg(test)]
#[test]
fn huge_numbers() {
    assert_eq!(
        big_add(
            b"100000000000000000000000000000000",
            b"100000000000000000000000000000000"
        ),
        b"200000000000000000000000000000000"
    );
}