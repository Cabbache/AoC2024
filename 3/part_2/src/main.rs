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
	let input = "do()".to_owned() + &fs::read_to_string("input").expect("Unable to read file");
	let mut s = 0;
	for en in input.split("do()").map(|sec| sec.split("don't()").next().unwrap()) {
		s += mult(en);
	}
	println!("{}",s);
}
