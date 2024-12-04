use std::fs;

fn main() {
	let input = fs::read_to_string("input").expect("Unable to read file");
	let mut v1 = Vec::new();
	let mut v2 = Vec::new();
	for line in input.trim().lines() {
		let mut ids = line.split_whitespace().map(|w| w.parse().unwrap());
		let (a,b): (usize, usize) = (ids.next().unwrap(), ids.next().unwrap());
		v1.push(a);
		v2.push(b);
	}
	let sum: usize = v1.iter().map(|left| v2.iter().filter(|&right| right==left).count()*left).sum();
	println!("{}", sum);
}
