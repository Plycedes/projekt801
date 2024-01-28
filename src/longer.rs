use std::io;
use std::fmt::Display;

fn longer_str<'a, T> (
	x: &'a str,
	y: &'a str,
	number: T
) -> &'a str 
where
	T: Display
{
	println!("{}", number);
	if x.len() > y.len() {
		return x;
	} else {
		return y;
	}
}

pub fn longer() {
	let mut str1 = String::new();
	let mut str2 = String::new();

	io::stdin().read_line(&mut str1).expect("Error");
	io::stdin().read_line(&mut str2).expect("Error");

	println!("{}",longer_str(&str1, &str2, 69));
}