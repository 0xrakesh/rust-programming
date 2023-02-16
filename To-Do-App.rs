use std::fs::File;
use std::fs;
use std::io;
use std::io::Write;
use std::io::Read;
use std::path::Path;
use std::io::prelude::*;
use std::io::{BufRead,BufReader};

fn main() {
	banner();
	loop {
		choice();
		let mut input = String::new();
		print!("Your Choice >  ");
		let _ = io::stdout().flush();
		io::stdin().read_line(&mut input).expect("Can't Read").to_string();
		if input.trim().eq("Exit") || input.trim().eq("4") {
			break;
		}	
		if input.trim().eq("New Job") || input.trim().eq("1") {
			let _ = create();
		}		
		if input.trim().eq("Delete Job") || input.trim().eq("2") {
			list();
			delete();
		}
		if input.trim().eq("List out the Jobs") || input.trim().eq("3") {
			list();
		} 
	}
}

fn create() {
	let path = Path::new("jobs.txt");
	if !path.exists() 
	{
		let _ = File::create("jobs.txt");
	}
	else {
		let mut file = File::options().append(true).open("jobs.txt").unwrap();
		let mut _job = String::new();
		print!("Job > ");
		io::stdout().flush();
		io::stdin().read_line(&mut _job);
		file.write(_job.as_bytes()).unwrap();
	}
}

fn list() {
	let path = Path::new("jobs.txt");
	if !path.exists() {
		println!("You don't have any jobs. So just create new jobs");
	}
	else {
		let mut file = File::open("jobs.txt").unwrap();
		let content = BufReader::new(file);
	        let mut count:usize = 1;
		println!("\n-----------------------------");
		println!("\tTo-Do List");
		println!("-----------------------------\n");
        	for line in content.lines() {
                	if let Ok(l) = line {
                	        println!("{}.{}",count,l);
                       	 	count += 1;
                	}
        	}
	}
}
fn delete() {
	let path = Path::new("jobs.txt");
	if !path.exists() {
		println!("You don't have any jobs. So just create new job");
	}
	else {
		let mut file = File::open("jobs.txt").unwrap();
		let mut del_content = String::new();
		print!("\nEnter the number to delete > ");
		io::stdout().flush();
		io::stdin().read_line(&mut del_content);
		let choice = del_content.trim().parse::<i32>().unwrap();
		let content = BufReader::new(file);
		let mut count:i32 = 1;
		let mut new_content: Vec<String> = vec![];
		for line in content.lines() {
			if let Ok(l) = line {
				if !count.eq(&choice) {
					new_content.push(l);
				}
				count += 1;
			}
		}
		fs::remove_file("jobs.txt");	
		let mut newfile = File::create("jobs.txt").unwrap();
		for i in new_content.iter() {
			newfile.write_all(i.as_bytes()).unwrap();
			newfile.write_all("\n".as_bytes()).unwrap();
		}
	}
}

fn choice() {
	println!("\n\n\tChoice");
	println!("-----------------------------");
	println!("1. New Job");
	println!("2. Remove Job");
	println!("3. List out the Jobs");
	println!("4. Exit");
	println!("-----------------------------\n");
}

fn banner() {
	println!("-------------------------------------------------------------");
	println!("\t\t\tTo-Do Application\t\t\t");
	println!("-------------------------------------------------------------");
}
