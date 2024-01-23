use std::io;

pub fn sum(){
	println!("Enter two numbers");	
	
	let mut s = String::new();
	io::stdin().read_line(&mut s).expect("Error");
	let x: u32 = s.trim().parse().unwrap();

	
	s = String::new();
	io::stdin().read_line(&mut s).expect("Error");
	let y: u32 = s.trim().parse().unwrap();
	
	println!("Sum = {}", x + y);
}