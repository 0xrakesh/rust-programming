use rand::Rng;

fn ban() {
	println!("--------------------------------------");
	println!("	    Password Generator          ");
	println!("--------------------------------------");
}

fn  main() {
	let mut rng = rand::thread_rng();
	let mut n1:usize;
	let mut length:u8 = 0;
	let alpha = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!@*_-");
	ban();
	print!("\n[+] Generated Password :");
	loop {
		n1 = rng.gen_range(0..=67);
		let ch: Option<char> = alpha.chars().nth(n1);
		match ch {
			Some(i) => { print!("{}",i); }
			None => { }
		};
		if length == 19 {
			break;
		}
		length += 1;
	}
	println!(" [+]\n");
}
