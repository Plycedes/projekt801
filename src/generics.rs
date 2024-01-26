struct Point<T, U>{
	x: T,
	y: U,
}

pub fn generics() {
	let v = vec![2, 4, 5, 3, 4, 6, 9];
	println!("{}", largest_val(v));

	let chars = vec!['A', 'F', 'X'];
	println!("{}", largest_val(chars));

	let mailibu = Point{
		x: 10.8,
		y: 15,
	};

	println!("{}, {}", mailibu.x, mailibu.y);
}

fn largest_val<T: PartialOrd + Copy>(list: Vec<T>) -> T {
	let mut max = list[0];

	for i in list {
		if i > max {
			max = i;
		}
	}

	return max;
}