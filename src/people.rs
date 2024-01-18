///Structure of person data
struct Person {
	name: String,
	age: u8,
	fav_color: String,
}

///Function for people
pub fn people() {
	///Stores data of people
	let people = vec![
		Person {
			name: "Luffy".to_owned(),
			age: 19,
			fav_color: "Red".to_owned(),
		},
		Person {
			name: "Zoro".to_owned(),
			age: 21,
			fav_color: "Green".to_owned(),
		},
		Person {
			name: "Jimbe".to_owned(),
			age: 55,
			fav_color: "Blue".to_owned(),
		},
	];

	///Prints the data of each person
	for person in people {
		if person.age <= 21 {
			println!("Name: {}", person.name);
			println!("Favorite color: {}", person.fav_color);
		}
	}
}