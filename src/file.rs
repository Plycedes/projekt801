use std::fs::File;
use std::io::ErrorKind;

pub fn file() {
	let _f = match File::open("example.txt"){
		Ok(file) => file,
		Err(e) => match e.kind(){
			ErrorKind::NotFound => match File::create("hello.text"){
				Ok(fc) => fc,
				Err(err) => panic!("Problem creating the file: {}", err),
			},
			_ => panic!("Unexpected error occured"),
		}		
	};	
}