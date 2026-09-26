use std::sync::Mutex;

fn main() {
	let counter = Mutex::new(0);
	
	{
		let mut num = counter.lock().unwrap();
		*num += 1;
		let mut nums2 = counter.lock().unwrap();
		*nums2 += 1;
}
	println!("counter = {}", *counter.lock().unwrap());

}
