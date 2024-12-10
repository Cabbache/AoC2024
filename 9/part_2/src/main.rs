use std::fs;

fn main() {
	let args = std::env::args().collect::<Vec<_>>();
  let file_path = std::path::Path::new(&args[1]);
	let input = fs::read_to_string(file_path).expect("Unable to read file");
	let input = input.trim();
	let digits: Vec<u8> = input.chars().map(|d| (d as u8)-('0' as u8)).collect();
	let cap = digits.iter().map(|&x| x as usize).sum();
	let mut mem: Vec<i32> = vec![0; cap];
	let mut ctr: usize = 0;
	let mut id: i32 = 0;
	let mut flip = true;
	for d in digits{
		for c in 0..d as usize{
			if flip {
				mem[ctr+c] = id;
			} else {
				mem[ctr+c] = -1; //-1 is empty space
			}
		}
		if flip {
			if d == 0 {
				println!("empty file");
			}
			id += 1;
		}
		ctr += d as usize;
		flip = !flip;
	}

	let mut r = cap-1;
	'outer: while r > 0 {
		/*
		println!("{}", mem.iter().map(|x| match &x {
			-1 => '.',
			_ => std::char::from_u32((*x as u32)+48).unwrap()
		}).collect::<String>());
		*/

		while mem[r] == -1 && r > 0{
			r -= 1;
		}
		if r == 0 {
			break;
		}
		let file_id = mem[r];
		let mut file_size = 0;
		while mem[r] == file_id {
			file_size += 1;
			r -= 1;
			if r <= 0 {
				break 'outer;
			}
		}
		for l in 0..r {
			if mem[l] != -1 {
				continue;
			}
			let mut empt_space = 0;
			while mem[l+empt_space] == -1 {
				empt_space += 1;
			}
			if file_size <= empt_space {
				for i in 0..file_size {
					mem[l+i] = file_id;
					mem[r+1+i] = -1;
				}
				break;
			}
		}
	}

	let res: i64 = mem.iter()
		.map(|&x| match x {
			-1 => 0,
			f => f,
		})
		.enumerate()
		.map(|(i,x)| (i as i64)*(x as i64)).sum();

	println!("{}", res);
}
