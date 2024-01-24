use std::io;

struct Rectangle{
	width: u32,
	height: u32,
}

impl Rectangle{
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

pub fn area(){
	let mut rect = Rectangle {width: 0, height: 0};
	let def = Rectangle {width: 6, height: 9};

	println!("Enter the dimensions of the rectangle");

	let mut input;	

	loop {
		input = String::new();
		io::stdin().read_line(&mut input).expect("Error occured");

		rect.width = match input.trim().parse(){
			Ok(num) => num,
			Err(_) => {
			println!("Please enter a valid dimension");
			continue;
			},
		};
		break;
	};		

	loop {
		input = String::new();
		io::stdin().read_line(&mut input).expect("Error occured");

		rect.height = match input.trim().parse(){
			Ok(num) => num,
			Err(_) => {
			println!("Please enter a valid dimension");
			continue;
			},
		};
		break;
	};

	println!("Area: {}", rect.area());
	println!("Given rectangle can hold default rectangle: {}", rect.can_hold(&def));
}
