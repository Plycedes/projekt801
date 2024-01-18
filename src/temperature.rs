///Structure to store temperature data
struct Temperature {
	degrees: f32,
}

///Implementation block for the Temperature structure
impl Temperature {
	///Prints temperature data
	fn print_temp(&self){
		println!("{} degrees celcius", self.degrees);
	}

	///Generates cold temperature data
	fn cold_temp() -> Self {
		Self {degrees: 2.1}
	}
}

///Function to manage temperature data
pub fn temperature() {
	let mut temp = Temperature {degrees: 45.3};
	temp.print_temp();

	temp = Temperature::cold_temp();
	temp.print_temp();
}