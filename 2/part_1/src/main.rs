use std::fs;

fn check_safe(rep: &Vec<i32>) -> bool {
	let diffs: Vec<i32> = rep.iter().zip(rep.iter().skip(1)).map(|(a,b)| b-a).collect();
	let polarity_ok = diffs.iter().all(|&d| d < 0) || diffs.iter().all(|&d| d > 0);
	let chg_ok = diffs.iter().map(|d| d.abs()).all(|d| d >= 1 && d <= 3);
	polarity_ok && chg_ok
}

fn main() {
	let input = fs::read_to_string("input").expect("Unable to read file");
	let mut ctr = 0;
	for line in input.trim().lines() {
		let nums: Vec<i32> = line.split_whitespace().map(|w| w.parse().unwrap()).collect();
		ctr += check_safe(&nums) as i32;
	}
	println!("{}", ctr);
}
