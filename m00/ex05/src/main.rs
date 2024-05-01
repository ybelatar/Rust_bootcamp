// fn is_leap_year(year: u32) -> bool {
// 	if year < 1 {panic!("Invalid year");}
// 	if year % 100 == 0 && year % 400 == 0 {return true;}
// 	if year % 100 == 0 && year % 400 != 0 {return false;}
// 	if year % 4 == 0 {return true;}
// 	false
// }

// fn num_days_in_month(year: u32, month: u32) -> u32 {
// 	if month < 1 || month > 12 || year < 1 {panic!("Invalid year or month");}
// 	if month == 2 && is_leap_year(year) {return 29;}
// 	if month == 2 && !is_leap_year(year) {return 28;}
// 	if month % 2 == 1 {return 31;}
// 	30
// }

fn is_leap_year(year: u32) -> bool {
	if year < 1 {panic!("Invalid year");}
	(year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}


fn num_days_in_month(year: u32, month: u32) -> u32 {
	match month {
		1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
		4 | 6 | 9 | 11 => 30,
		2 if is_leap_year(year) => 29,
		_ => 28
	}
}

fn get_month(month: u32) -> String {
	match month {
		1 => String::from("January"),
		2 => String::from("February"),
		3 => String::from("March"),
		4 => String::from("April"),
		5 => String::from("May"),
		6 => String::from("June"),
		7 => String::from("July"),
		8 => String::from("August"),
		9 => String::from("September"),
		10 => String::from("October"),
		11 => String::from("November"),
		_ => String::from("December")
	}
}

fn main() {
	let current_year = 6;
	let mut day = 1;
	for year in 1..=current_year {
		for month in 1..=12 {
			for days in 1..=num_days_in_month(year, month) {
				if day == 5 && days == 13 {
					println!("Friday, {0} 13, {1}", get_month(month), year);
				}
				day = match day + 1 {
					8 => 1,
					_ => day + 1
				};
			}
		}
	}
}


#[cfg(test)]
mod tests {
    use crate::{is_leap_year, num_days_in_month};

	#[test]
	fn test_leap_1600() {assert_eq!(is_leap_year(1600), true);}
	#[test]
	fn test_leap_1500() {assert_eq!(is_leap_year(1500), false);}
	#[test]
	fn test_leap_2004() {assert_eq!(is_leap_year(2004), true);}
	#[test]
	fn test_leap_2003() {assert_eq!(is_leap_year(2003), false);}
	#[test]
	#[should_panic]
	fn test_leap_0() {assert!(is_leap_year(0));}
	#[test]
	fn test_month_28() {assert_eq!(num_days_in_month(1500, 2), 28);}
	#[test]
	fn test_month_29() {assert_eq!(num_days_in_month(1600, 2), 29);}
	#[test]
	fn test_month_30() {assert_eq!(num_days_in_month(1600, 4), 30);}
	#[test]
	fn test_month_31() {assert_eq!(num_days_in_month(1600, 5), 31);}
	#[test]
	#[should_panic]
	fn test_month_panic() {assert_eq!(num_days_in_month(1600, 13), 31);}
}