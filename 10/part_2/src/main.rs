use std::fs;

type Mm = Vec<Vec<u8>>;

fn count(h: i32, prev: Option<(usize,usize)>, pos: (usize, usize), map: &Mm) -> i32{
	if map[pos.0][pos.1] == 9 {
		return 1;
	}
	let moves = vec![(0,1),(1,0), (-1,0), (0,-1)];
	let mut c = 0;
	for d in moves {
		let next = ((pos.0 as i32)+d.0,(pos.1 as i32)+d.1);
		if next.0 < 0 || next.1 < 0 || next.0 >= (map.len() as i32) || next.1 >= (map.len() as i32){
			continue;
		}
		let unext = (next.0 as usize, next.1 as usize);
		if Some(unext) == prev { //don't walk backwards
			continue;
		}
		if map[unext.0][unext.1] as i32 != h+1 { //next step must increase
			continue;
		}
		c += count(h+1, Some(pos), unext, &map);
	}
	c
}

fn main() {
	let args = std::env::args().collect::<Vec<_>>();
  let file_path = std::path::Path::new(&args[1]);
	let input = fs::read_to_string(file_path).expect("Unable to read file");
	let input = input.trim();
	
	let map: Mm = input.lines().map(|l| l.chars().map(|c| (c as u8)-48).collect::<Vec<u8>>()).collect();
	let mut c = 0;
	for i in 0..map.len() {
		for j in 0..map[0].len() {
			if map[i][j] == 0 {
				c += count(0, None, (i,j), &map);
			}
		}
	}
	println!("{}", c);
}
