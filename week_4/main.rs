use std::thread;

fn main(){
	let handle = thread::spawn(|| {
	
	for i in 1..5 {
		println!("hi number {} from the spawned thread!", i);}
	});
	
	let handle1 = thread::spawn(||{println!("H");});
	let handle2 = thread::spawn(||{println!("E");});

	handle.join().unwrap();
	handle1.join().unwrap();
	handle2.join().unwrap();
	println!("done in main");
}
