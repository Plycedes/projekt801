struct Temperature {
	degrees: f32,
}

impl Temperature {	
	fn print_temp(&self){
		println!("{} degrees celcius", self.degrees);
	}
	
	fn cold_temp() -> Self {
		Self {degrees: 2.1}
	}
}

pub fn temperature() {
	let mut temp = Temperature {degrees: 45.3};
	temp.print_temp();

	temp = Temperature::cold_temp();
	temp.print_temp();
}