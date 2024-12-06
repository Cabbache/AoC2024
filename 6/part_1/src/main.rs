use std::fs;

fn main() {
	let args = std::env::args().collect::<Vec<_>>();
  let file_path = std::path::Path::new(&args[1]);
	let input = fs::read_to_string(file_path).expect("Unable to read file");
	let input = input.trim();
	let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();
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

	println!("{} x {}", map[0].len(), map.len());
	println!("{:?} {}",pos,map[pos.0 as usize][pos.1 as usize]);
	let mut direction: (i32,i32) = (-1,0);
	let mut covered = vec![vec![0; map[0].len()]; map.len()];
	covered[pos.0 as usize][pos.1 as usize] = 1;
	loop {
		let next = (pos.0+direction.0, pos.1+direction.1);
		if next.0 < 0 || next.0 >= map.len() as i32 || next.1 < 0 || next.1 >= map.len() as i32 {
			break;
		}
		match map[next.0 as usize][next.1 as usize] {
			'.' => {
				pos = next;
				covered[pos.0 as usize][pos.1 as usize] = 1;
			}
			'#' => direction = match direction {
				(0,1) => (1,0),
				(1,0) => (0,-1),
				(0,-1) => (-1,0),
				(-1,0) => (0,1),
				_ => (0,0),
			},
			_ => eprintln!("??"),
		}
	}
	let sum: usize = covered.iter().map(|l| l.iter().sum::<usize>()).sum();
	println!("{}",sum);
}
