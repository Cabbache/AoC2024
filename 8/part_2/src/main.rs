use std::fs;
use std::collections::HashSet;

type Mm = Vec<Vec<char>>;

fn main() {
	let args = std::env::args().collect::<Vec<_>>();
  let file_path = std::path::Path::new(&args[1]);
	let input = fs::read_to_string(file_path).expect("Unable to read file");
	let input = input.trim();
	
	let map: Mm = input
		.lines()
		.map(|l| l.chars().collect::<Vec<char>>())
		.collect();

	let mut mapclone = map.clone();
	
	let bands: HashSet<char> = HashSet::from_iter(input.chars().filter(|&c| c != '.'));
	for band in bands {
		let mut posl: Vec<(i32, i32)> = Vec::new();
		for i in 0..map.len() {
			for j in 0..map[0].len() {
				if map[i][j] == band {
					posl.push((i as i32,j as i32));
				}
			}
		}
		for (a,b) in posl.iter().flat_map(|p| posl.iter().filter(move |&q| p != q).map(move |q| (p,q))) {
			let diff = (a.0 - b.0, a.1 - b.1);
			let within = |p: &(i32, i32)| p.0 >= 0 && p.1 >= 0 && p.0 < map.len() as i32 && p.1 < map[0].len() as i32;
			for an in (-200..200).map(|mul| (a.0 + mul*diff.0, a.1 + mul*diff.1)).filter(within) {
				mapclone[an.0 as usize][an.1 as usize] = '#';
			}
		}
	}
	for row in &mapclone {
		println!("{}", row.iter().collect::<String>());
	}
	let c: usize = mapclone.iter().map(|row| row.iter().filter(|&c| *c == '#').count()).sum();
	println!("{}",c);
}
