use std::io;

fn main() {
	let mut name = String::new();
	println!("Enter the name :");
	io::stdin().read_line(&mut name);
	println!("Name : {name}");
}
