use std::io;

struct Rectangle{
	width: u32,
	height: u32,
}

pub fn area(){
	let mut rect = Rectangle {width: 0, height: 0};

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

	println!("Area: {}", res(&rect));
}

fn res(rectangle: &Rectangle) -> u32 {
		rectangle.width * rectangle.height
}