use std::fs;

fn main() {
	let input = fs::read_to_string("input").expect("Unable to read file");
	let mut v1 = Vec::new();
	let mut v2 = Vec::new();
	for line in input.trim().lines() {
		let mut ids = line.split_whitespace().map(|w| w.parse().unwrap());
		let (a,b): (i32, i32) = (ids.next().unwrap(), ids.next().unwrap());
		v1.push(a);
		v2.push(b);
	}
	v1.sort();
	v2.sort();
	let sum: i32 = v1.iter().zip(v2.iter()).map(|(a,b)| (a-b).abs()).sum();
	println!("{}",sum);
}
