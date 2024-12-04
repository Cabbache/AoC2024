use std::fs;
use regex::Regex;

fn mult(input: &str) -> i64 {
	let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
	let mut s = 0;
	for (_, [a,b]) in re.captures_iter(input).map(|c| c.extract()) {
		s += a.parse::<i64>().unwrap()*
		b.parse::<i64>().unwrap()
	}
	s
}

fn main() {
	let input = fs::read_to_string("input").expect("Unable to read file");
	println!("{}",mult(&input));
}
