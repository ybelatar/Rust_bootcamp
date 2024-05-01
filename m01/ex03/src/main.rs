fn check_in(big: &[u32], small: &[u32]) -> bool {
	for i in big {
		if !small.contains(i) {return false;}
	}
	true
}

fn largest_group<'a, 'b>(haystack: &'a [u32], needle: &'b [u32]) -> &'a [u32] {
	if needle.is_empty() {return &[];}
	for i in (1..haystack.len()).rev() {
		let mut start = 0;
		while start + i < haystack.len() {
			// println!("Debug loop {:#?}", &haystack[start..=(start + i)]);
			if check_in(&haystack[start..=(start + i)], needle) {return &haystack[start..=(start + i)];}
			start += 1;
		}
	}
	&[]
}



fn main() {
    println!("{:#?}", largest_group(&[1, 3, 4, 3, 5, 5, 4], &[1, 3, 4]));
}


#[cfg(test)]
#[test]
fn test1() {assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);}
#[test]
fn test2() {assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);}
#[test]
fn test3() {assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);}
#[test]
fn test4() {assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);}
#[test]
fn test_lifetimes() {
    let haystack = [1, 2, 3, 2, 1];
    let result;

    {
        let needle = [2, 3];
        result = largest_group(&haystack, &needle);
    }

    assert_eq!(result, &[2, 3, 2]);
}
