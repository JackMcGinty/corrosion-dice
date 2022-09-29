pub mod command_functions {
	// use std::io::Write;
	use std::{thread, time};
	use std::io::Write;
	use rand::Rng;
	pub fn roll20() -> i32 {
		print!("rolling... [ ]\r");
		std::io::stdout().flush().expect("couldn't flush the display");
		thread::sleep(time::Duration::from_millis(100));
		// This is the number of times that the 'die' will change before settling on the final result
		let target = rand::thread_rng().gen_range(4..=20);
		let mut i = 0;
		while i < target {
			let temp_result = rand::thread_rng().gen_range(1..=20);
			print!("rolling... [ {temp_result} ]           \r");
			std::io::stdout().flush().expect("couldn't flush the display");
			thread::sleep(time::Duration::from_millis(i * 30));
			i += 1;
		}
		let final_result = rand::thread_rng().gen_range(1..=20); // generate random integer between 1 and 20
		println!("result: [ {final_result} ]                 ");
		// the extra spaces are to make sure to overwrite the previous output
		final_result // We might need to use this at some point.
	}
}