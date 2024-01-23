struct Locker {
	name: String,
	id: Option<u32>,
}

pub fn lockers() {	
	let lockers = vec![
		Locker {
			name: "Erwin".to_owned(),
			id: Some(69),
		},
		Locker {
			name: "Levi".to_owned(),
			id: None,
		}
	];
	
	for locker in lockers {
		println!("Name: {}", locker.name);
		match locker.id {
			Some(id) => println!("Locker id: {}", id),
			None => println!("Locker id not provided"),
		}
	}
}