pub fn cords(){
	let (x, y) = get_cords();
	println!("{}, {}",x,y);

	if y > 5 {
		println!("Greater");
	}
	else if y < 5 {
		println!("Lesser");
	}
	else{
		println!("Equal");
	}
}
fn get_cords() -> (i32, i32){
	return (-52, 48);	
}