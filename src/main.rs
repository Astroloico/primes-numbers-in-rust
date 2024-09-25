use std::str;
use std::io::*;
use std::vec;
use std::f32;

fn main() {
	let mut input:String = String::new();
	stdin().read_line(&mut input).unwrap();
	input = input.trim().parse().unwrap();
	let n:usize = str::parse::<usize>(&input).unwrap_or_else(|e| {
		panic!("{}", e); });
	let mut bool_vec:Vec<bool> = vec![true; n];
	let n_sqrt:usize = f32::sqrt(n as f32).ceil() as usize;
	for i in 2..n_sqrt + 1 {
		if bool_vec[i] {
			print!("{} ", i);
			for j in i*i..n {
				if j % i == 0 {
					bool_vec[j] = false;
				}
			}
		}
	} for i in n_sqrt + 1..n {
		if bool_vec[i] {
			print!("{} ", i);
		}
	}
}