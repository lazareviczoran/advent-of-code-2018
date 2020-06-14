use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let nanobots = read_input("input.txt");
  println!(
    "Day 23: Experimental Emergency Teleportation part1 solution\n{:?}",
    find_in_range_for_strongest_nanobot(&nanobots)
  );

  println!(
    "Day 23: Experimental Emergency Teleportation part2 solution\n{:?}",
    find_dist_of_coord_that_covers_most_bots(&nanobots)
  );
}

fn find_dist_of_coord_that_covers_most_bots(nanobots: &Vec<Nanobot>) -> i64 {
  let mut ranges_bounds = BTreeMap::new();
  for nb in nanobots.iter() {
    let dist = nb.pos.0 + nb.pos.1 + nb.pos.2;
    *ranges_bounds.entry(dist - nb.radius).or_insert(0) += 1;
    *ranges_bounds.entry(dist + nb.radius + 1).or_insert(0) -= 1;
  }
  let mut intervals = Vec::new();
  let mut prev = Vec::new();
  let mut max = i64::min_value();
  let mut val = 0;
  for (i, (from, count)) in ranges_bounds.iter().enumerate() {
    val += count;
    if max < val {
      max = val;
    }
    prev.push((from, val));
    if i > 0 {
      intervals.push(((prev[i - 1].0, from - 1), prev[i - 1].1));
    }
  }

  let max_intervals = intervals.iter().filter(|a| a.1 == max);
  let mut min_dist = i64::max_value();
  for interval in max_intervals {
    let (from, to) = interval.0;
    let val = if from.signum() == to.signum() {
      from.abs().min(to.abs())
    } else {
      return 0;
    };
    if min_dist > val {
      min_dist = val;
    }
  }
  min_dist
}

fn find_in_range_for_strongest_nanobot(nanobots: &Vec<Nanobot>) -> i64 {
  let mut strongest = nanobots[0];
  for i in 1..nanobots.len() {
    if nanobots[i].radius > strongest.radius {
      strongest = nanobots[i];
    }
  }
  let mut in_range = 0;
  for bot in nanobots.iter() {
    if strongest.is_in_range(bot) {
      in_range += 1;
    }
  }
  in_range
}

fn read_input(filename: &str) -> Vec<Nanobot> {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");
  let re = Regex::new(r"(-?\d+)").unwrap();

  let mut res = Vec::new();
  for s in content.split_terminator('\n') {
    let mut caps = vec![];
    for c in re.captures_iter(s) {
      caps.push(c[1].parse::<i64>().unwrap());
    }
    res.push(Nanobot::new((caps[0], caps[1], caps[2]), caps[3]));
  }

  res
}

fn calculate_distance(n1: &Nanobot, n2: &Nanobot) -> i64 {
  (n1.pos.0 - n2.pos.0).abs()
    + (n1.pos.1 - n2.pos.1).abs()
    + (n1.pos.2 - n2.pos.2).abs()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Nanobot {
  pos: (i64, i64, i64),
  radius: i64,
}
impl Nanobot {
  pub fn new(pos: (i64, i64, i64), radius: i64) -> Self {
    Self { pos, radius }
  }

  pub fn is_in_range(&self, other: &Nanobot) -> bool {
    calculate_distance(self, other) <= self.radius
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_test() {
    let nanobots = read_input("test-input.txt");
    assert_eq!(find_in_range_for_strongest_nanobot(&nanobots), 7);
  }

  #[test]
  fn part2_test() {
    let nanobots = read_input("test-input2.txt");
    assert_eq!(find_dist_of_coord_that_covers_most_bots(&nanobots), 36);
  }
}
