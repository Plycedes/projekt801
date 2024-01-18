///Vector demonstration
pub fn vectors(){
	let nums = vec![10, 20, 30, 40];

	///Prints each value in the vector;
	for n in nums {
		if n == 30 {
			println!("Thirty");
		}else{
			println!("{}", n);
		}
	}
}