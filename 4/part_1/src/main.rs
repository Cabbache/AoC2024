use std::fs;

fn transpose(lines: &Vec<String>) -> Vec<String> {
	let linelen = lines[0].chars().count();
	(0..linelen)
		.map(|index| {
			lines
				.iter()
				.map(move |line| line.chars().nth(index).unwrap())
				.collect::<String>()
		})
		.collect()
}

fn flip_horizontal(lines: &Vec<String>) -> Vec<String> {
	lines
		.iter()
		.map(|line| line.chars().rev().collect())
		.collect()
}

fn half_diagonal(lines: &Vec<String>) -> Vec<String> {
	let linelen = lines[0].chars().count();
	let mut diagonals = Vec::new();
	let mut ctr = 0;
	while ctr <= linelen {
		let diagonal: String = (0..ctr)
			.map(|index| {
				lines
					.iter()
					.nth(linelen-index-1)
					.unwrap()
					.chars().nth(ctr-index-1).unwrap()
			})
			.collect();
		diagonals.push(diagonal);
		ctr += 1;
	}
	diagonals
}

fn half_diagonal2(lines: &Vec<String>) -> Vec<String> {
	let linelen = lines[0].chars().count();
	let mut diagonals = Vec::new();
	let mut ctr = 0;
	while ctr <= linelen {
		let diagonal: String = (0..ctr)
			.map(|index| {
				lines
					.iter()
					.nth(index)
					.unwrap()
					.chars().nth(ctr-index-1).unwrap()
			})
			.collect();
		diagonals.push(diagonal);
		ctr += 1;
	}
	diagonals
}

fn countlines(lines: &Vec<String>) -> usize {
	let count = |string: &String| {
		string
			.as_bytes()
			.windows(4)
			.filter(|&w| w == "XMAS".as_bytes())
			.count()
	};
	let countall = |lines: &Vec<String>| {
		lines
			.iter()
			.map(|string| count(&string.to_string()))
			.sum::<usize>()
	};
	countall(lines) + countall(&flip_horizontal(lines))
}

fn main() {
	let input = fs::read_to_string("input").expect("Unable to read file");
	let input = input.trim();
	let lines = input
		.split_whitespace()
		.map(|s| s.to_string())
		.collect::<Vec<String>>();
	let rotated_lines = transpose(&lines);

	let diagonal_lines_left_half1 = half_diagonal(&lines);
	let mut diagonal_lines_right_half1 = half_diagonal(&flip_horizontal(&transpose(&flip_horizontal(&rotated_lines))));
	diagonal_lines_right_half1.pop();

	let diagonal_lines_left_half = half_diagonal2(&lines);
	let mut diagonal_lines_right_half = half_diagonal2(&flip_horizontal(&transpose(&flip_horizontal(&rotated_lines))));
	diagonal_lines_right_half.pop();

	let hc = countlines(&lines);
	let vc = countlines(&rotated_lines);
	let dlh1 = countlines(&diagonal_lines_left_half1);
	let drh1 = countlines(&diagonal_lines_right_half1);
	let dlh2 = countlines(&diagonal_lines_left_half);
	let drh2 = countlines(&diagonal_lines_right_half);

	println!("{} {} {} {} {} {}",hc,vc,dlh1,drh1,dlh2,drh2);

	let total = hc+vc+dlh1+drh1+dlh2+drh2;
	println!("{}",total);
}
