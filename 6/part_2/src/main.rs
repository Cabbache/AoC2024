use std::fs;
use std::collections::HashSet;

fn main() {
	let args = std::env::args().collect::<Vec<_>>();
  let file_path = std::path::Path::new(&args[1]);
	let input = fs::read_to_string(file_path).expect("Unable to read file");
	let input = input.trim();
	let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();
	let mut pos: (i32,i32) = (0,0);
	'outer: for i in 0..map.len() {
		for j in 0..map[0].len() {
			if "#.".contains(map[i][j]) {
				continue;
			}
			pos = (i as i32,j as i32);
			break 'outer;
		}
	}
	map[pos.0 as usize][pos.1 as usize] = '.';

	println!("{} x {}", map[0].len(), map.len());
	println!("{:?} {}",pos,map[pos.0 as usize][pos.1 as usize]);
	let mut it = (0..map.len()).flat_map(|i| (0..map.len()).map(move |j| (i,j)));
	//let mut it = (0..map.len()).zip(0..map.len());
	let mut obst = 0;
	let oldpos = pos;
	while let Some((xo,yo)) = it.next() {
		if map[xo][yo] != '.' {
			continue;
		}
		pos = oldpos;
		let mut mapc = map.clone();
		mapc[xo][yo] = '#';
		let mut direction: (i32,i32) = (-1,0);
		let mut covered = vec![vec![HashSet::new(); mapc[0].len()]; mapc.len()];
		covered[pos.0 as usize][pos.1 as usize].insert(direction);
		loop {
			let next = (pos.0+direction.0, pos.1+direction.1);
			if next.0 < 0 || next.0 >= mapc.len() as i32 || next.1 < 0 || next.1 >= mapc.len() as i32 {
				break;
			}
			match mapc[next.0 as usize][next.1 as usize] {
				'.' => {
					pos = next;
					if covered[pos.0 as usize][pos.1 as usize].contains(&direction) {
						obst += 1;
						break;
					} else {
						covered[pos.0 as usize][pos.1 as usize].insert(direction);
					}
				}
				'#' => direction = match direction {
					(0,1) => (1,0),
					(1,0) => (0,-1),
					(0,-1) => (-1,0),
					(-1,0) => (0,1),
					_ => (0,0),
				},
				x => eprintln!("?? {}", x),
			}
			//for c in &covered {
			//	println!("{}", c.iter().map(|t| t.len().to_string()).collect::<String>());
			//}
			//println!("---");
		}
	}
	println!("{}",obst);
}
