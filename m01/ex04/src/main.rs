fn cmp_boxes(b1: &[u32; 2], b2: &[u32; 2]) -> bool {
	if b1[0] <= b2[0] && b1[1] <= b2[1] {return false;}
	else if b1[0] >= b2[0] && b1[1] >= b2[1] {return true;}
	panic!("Boxes don't fit into one another {:#?}, {:#?}", b1, b2);
}


fn insert(boxes: &mut [[u32; 2]], j: usize) {
	let mut k = j;
	while k > 0 && cmp_boxes(&boxes[k], &boxes[k - 1]) {
		boxes.swap(k, k - 1);
		k -= 1; 
	}
}

fn sort_boxes(boxes: &mut [[u32; 2]]) {
	for k in 1..boxes.len() {
		insert(boxes, k);
	}
}

fn main() {
	let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
	sort_boxes(&mut boxes);
	println!("{:#?}", boxes);
	// assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
    // println!("Hello, world!");
}





#[cfg(test)]
#[test]
fn test1() {
	let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
	sort_boxes(&mut boxes);
	// println!("{:#?}", boxes);
	assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
}
#[test]
#[should_panic]
fn test_panic() {
	let mut boxes = [[3, 3], [4, 3], [1, 0], [1, 100], [3, 3]];
	sort_boxes(&mut boxes);
	assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
}