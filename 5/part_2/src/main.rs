use std::collections::HashSet;
use std::fs;

fn main() {
  let input = fs::read_to_string("input").expect("Unable to read file");
  let mut sum = 0;
  let mut rules: HashSet<(i32, i32)> = HashSet::new();
  for line in input.trim().lines() {
    if line.contains('|') {
      let mut sp = line.split('|');
      let l: i32 = sp.next().unwrap().parse().unwrap();
      let r: i32 = sp.next().unwrap().parse().unwrap();
      rules.insert((l, r));
    } else if line.contains(',') {
      let mut nums: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

			let bad = nums
				.iter()
				.enumerate()
				.any(|(i, &n)| nums.iter().skip(i + 1).any(|&c| rules.contains(&(c, n))));
			
			if !bad {
				continue;
			}

      let compare = |a: &i32,b: &i32| {
        use std::cmp::Ordering::*;
        match (rules.contains(&(*a,*b)), rules.contains(&(*b,*a))) {
          (false, true) => Less,
          (true,false) => Greater,
          _ => Equal,
        }
      };
      nums.sort_by(compare);
      sum += nums[nums.len()/2]
    }   
  }
  println!("{}", sum);
}
