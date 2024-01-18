use std::io;

///Function to print sum of the two numbers
pub fn sum(){
	println!("Enter two numbers");
	
	///Takes user input for the numbers
	let mut s = String::new();
	io::stdin().read_line(&mut s).expect("Error");
	let x: u32 = s.trim().parse().unwrap();

	
	s = String::new();
	io::stdin().read_line(&mut s).expect("Error");
	let y: u32 = s.trim().parse().unwrap();

	///Prints the sum
	println!("Sum = {}", x + y);
}