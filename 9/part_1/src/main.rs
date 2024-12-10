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

	let mut last = cap-1;
	for i in 0..cap {
		if mem[i] != -1 {
			continue;
		}
		while mem[last] == -1 && last > 0{
			last -= 1;
		}
		if last == 0 || i >= last {
			break;
		}
		mem[i] = mem[last];
		mem[last] = -1;
	}

	let res: i64 = mem.iter()
		.filter(|&x| *x != -1)
		.enumerate()
		.map(|(i,&x)| (i as i64)*(x as i64)).sum();

	println!("{}", res);
}
