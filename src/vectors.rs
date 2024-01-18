pub fn vectors(){
	let nums = vec![10, 20, 30, 40];

	for n in nums {
		if n == 30 {
			println!("Thirty");
		}else{
			println!("{}", n);
		}
	}
}