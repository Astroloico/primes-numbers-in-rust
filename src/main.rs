use std::str;
use std::io::*;

fn main() {
	let mut input:String = String::new();
	stdin().read_line(&mut input).unwrap();
	input = input.trim().parse().unwrap();
	println!("You typed: {}", input.trim());

	let n:i64 = str::parse::<i64>(&input).unwrap_or_else(|e| {
		panic!("{}", e);
	});
	println!("You typed: {}", n);
}