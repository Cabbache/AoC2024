use std::fs;

fn main() {
	let input = fs::read_to_string("input").expect("Unable to read file");
	let input = input.trim();
	let charmap = input
		.split_whitespace()
		.map(|s| s.chars().collect::<Vec<char>>())
		.collect::<Vec<Vec<char>>>();

	let searchmaps = [
		[
			['M','.','M'],
			['.','A','.'],
			['S','.','S'],
		],
		[
			['S','.','M'],
			['.','A','.'],
			['S','.','M'],
		],
		[
			['S','.','S'],
			['.','A','.'],
			['M','.','M'],
		],
		[
			['M','.','S'],
			['.','A','.'],
			['M','.','S'],
		],
	];

	let mut matches = 0;

	for i in 0..charmap.len()-2 {
		for j in 0..charmap[0].len()-2 {
			for map in searchmaps {
				let mut equal = true;
				'kernel: for a in 0..3 {
					for b in 0..3 {
						if map[a][b] == '.' {
							continue;
						}
						equal = map[a][b] == charmap[i+a][j+b];
						if !equal {
							break 'kernel;
						}
					}
				}
				matches += equal as usize;
			}
		}
	}

	println!("{}", matches);
}
