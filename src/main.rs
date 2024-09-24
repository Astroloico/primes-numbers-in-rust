use std::str;
use std::io::*;
use std::vec;
use std::f32;

fn main() {
	let mut input:String = String::new();
	stdin().read_line(&mut input).unwrap();
	input = input.trim().parse().unwrap();
	println!("You typed: {}", input.trim());
	let n:usize = str::parse::<usize>(&input).unwrap_or_else(|e| {
		panic!("{}", e); });
	let mut bool_vec:Vec<bool> = vec![true; n];
	let n_sqrt:usize = f32::sqrt(n as f32).ceil() as usize;
	for i in 0..n_sqrt {
		print!("{} ", bool_vec[i]);
	}
}