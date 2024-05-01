pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
	true
}




#[cfg(test)]
mod tests {
	use crate::strpcmp;

	#[test]
	fn test1() {assert!(strpcmp(b"abc", b"abc"));}
	#[test]
	fn test2() {assert!(strpcmp(b"abcd", b"ab*"));}
	#[test]
	fn test3() {assert!(!strpcmp(b"cab", b"ab*"));}
	#[test]
	fn test4() {assert!(strpcmp(b"dcab", b"*ab"));}
	#[test]
	fn test5() {assert!(!strpcmp(b"abc", b"*ab"));}
	#[test]
	fn test6() {assert!(strpcmp(b"ab000cd", b"ab*cd"));}
	#[test]
	fn test7() {assert!(strpcmp(b"abcd", b"ab*cd"));}
	#[test]
	fn test8() {assert!(strpcmp(b"", b"****"));}
}