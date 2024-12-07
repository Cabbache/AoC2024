use std::fs;
use std::collections::HashSet;

fn getres(pp: &Vec<i64>, ops: Vec<char>) -> i64 {
	pp.iter().skip(1).zip(ops.iter()).fold(pp[0], |acc, (n,op)| match op {
		'+' => acc + n,
		'*' => acc * n,
		_ => panic!()
	})
}

fn check_op(res: i64, pp: &Vec<i64>, allowed: &Vec<char>) -> bool {
	let npos = pp.len()-1;
	let nperms = allowed.len().pow(npos as u32);
	for mut i in 0..nperms {
		let mut idxs = Vec::new();
		let mut pow: i64 = (npos-1).try_into().unwrap();
		while pow >= 0 {
			let modiv = allowed.len().pow(pow as u32);
			let idx = i/modiv;
			idxs.push(idx);
			i %= modiv;
			pow -= 1;
		}
		let ops: Vec<char> = idxs.iter().map(|&id| *allowed.get(id).unwrap()).collect();
		if getres(pp, ops) == res {
			return true
		}
	}
	false
}

fn main() {
	let args = std::env::args().collect::<Vec<_>>();
  let file_path = std::path::Path::new(&args[1]);
	let input = fs::read_to_string(file_path).expect("Unable to read file");
	let input = input.trim();
	let total: i64 = input.lines().map(|l| {
		let mut ss = l.split(":").map(|s| s.trim());
		let res = ss.next().unwrap().parse::<i64>().unwrap();
		let pp = ss.next().unwrap().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
		(res,pp)
	}).filter(|(res,pp)| check_op(*res,pp,&vec!['+','*'])).map(|(res,_)| res).sum();
	println!("{}",total);
}
