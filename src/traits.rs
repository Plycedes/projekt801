struct Tweet{
	username: String,
	content: String,
	date: String,
	views: u32,
}

struct Article{
	author: String,
	headline: String,
	date: String,
	reads: u32,
}

trait Printing{
	fn printing(&self) -> String;
}

trait Stats{
	fn stats(&self) -> String;
}

impl Printing for Tweet{
	fn printing(&self) -> String {
		format!("{}: {};", self.username, self.content)
	}
}

impl Printing for Article{
	fn printing(&self) -> String {
		format!("{}: {};", self.author, self.headline)
	}
}

impl Stats for Tweet{
	fn stats(&self) -> String {
		format!("{}, {} views", self.date, self.views)
	}
}

impl Stats for Article{
	fn stats(&self) -> String {
		format!("{}, {} reads", self.date, self.reads)
	}
}

fn data<T>(item: &T) -> String where T: Stats + Printing {
	format!("{} {}", item.printing(), item.stats())
}

fn new_content() -> impl Printing + Stats {
	Article{
		author: "Pratap".to_string(),
		headline: "Rust is an ok lanuage".to_string(),
		date: "14th March".to_string(),
		reads: 781,
	}
}

pub fn traits(){
	let tweet = Tweet{
		username: "plycedes".to_string(),
		content: "Rust is a good language".to_string(),
		date: "27th Jan".to_string(),
		views: 279,
	};

	let article = Article{
		author: "Abhay".to_string(),
		headline: "Rust is a bad language".to_string(),
		date: "09th Feb".to_string(),
		reads: 148,
	};

	println!("{}", data(&tweet));
	println!("{}", data(&article));
	println!("{}", data(&new_content()));	
}