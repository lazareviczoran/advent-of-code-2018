#![feature(map_first_last)]
use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let (pots, patterns) = read_input("input.txt");
  println!(
    "Day 12: Subterranean Sustainability part1 solution\n{:?}",
    find_sum_after_20th_gen(&pots, &patterns)
  );

  println!(
    "Day 12: Subterranean Sustainability part2 solution\n{:?}",
    find_sum_after_50_billion_iterations(&pots, &patterns)
  );
}

fn find_sum_after_50_billion_iterations(
  pots: &BTreeMap<i32, char>,
  patterns: &BTreeMap<String, char>,
) -> i64 {
  let mut curr_pots = pots.clone();
  let mut prev_sum = 0;
  let mut prev_sum_diff = 0;
  let mut sum = calculate_sum(pots);
  let mut sum_diff = sum - prev_sum;
  let mut total_sum = sum as i64;
  let mut i = 0i64;
  let mut repeat_num = 0;
  while sum_diff != prev_sum_diff || repeat_num < 10 {
    if sum_diff == prev_sum_diff {
      repeat_num += 1;
    } else {
      repeat_num = 0;
    }
    prev_sum = sum;
    prev_sum_diff = sum_diff;
    apply_transformations(&mut curr_pots, patterns);
    sum = calculate_sum(&curr_pots);
    sum_diff = sum - prev_sum;
    total_sum += sum_diff as i64;
    i += 1;
  }

  let remaining_iteration = 50_000_000_000i64 - i;
  total_sum += remaining_iteration * sum_diff as i64;
  total_sum
}

fn find_sum_after_20th_gen(
  pots: &BTreeMap<i32, char>,
  patterns: &BTreeMap<String, char>,
) -> i32 {
  let mut curr_pots = pots.clone();

  for _i in 0..20 {
    apply_transformations(&mut curr_pots, patterns);
  }

  calculate_sum(&curr_pots)
}

fn calculate_sum(pots: &BTreeMap<i32, char>) -> i32 {
  pots.iter().fold(0, |acc, (pot_pos, pot_val)| {
    if *pot_val == '#' {
      return acc + pot_pos;
    }
    acc
  })
}

fn apply_transformations(
  pots: &mut BTreeMap<i32, char>,
  patterns: &BTreeMap<String, char>,
) {
  let mut current_pots = pots.clone();
  // expand the pots
  let (first_pos, _) = current_pots.first_key_value().unwrap();
  let mut first_plant_pos = *first_pos;
  while current_pots.get(&first_plant_pos) == Some(&'.') {
    first_plant_pos += 1;
  }

  let (last_pos, _) = current_pots.last_key_value().unwrap();
  let mut last_plant_pos = *last_pos;
  while current_pots.get(&last_plant_pos) == Some(&'.') {
    last_plant_pos -= 1;
  }
  for i in 1..=5 {
    pots.insert(first_plant_pos - i, '.');
    pots.insert(last_pos + i, '.');
  }

  current_pots = pots.clone();
  let keys = current_pots.keys();
  for pos in keys {
    let mut pot_range = String::new();
    pot_range.push(*current_pots.get(&(pos - 2)).unwrap_or(&'.'));
    pot_range.push(*current_pots.get(&(pos - 1)).unwrap_or(&'.'));
    pot_range.push(*current_pots.get(&(pos)).unwrap());
    pot_range.push(*current_pots.get(&(pos + 1)).unwrap_or(&'.'));
    pot_range.push(*current_pots.get(&(pos + 2)).unwrap_or(&'.'));
    if let Some(new_val) = patterns.get(&pot_range) {
      pots.insert(*pos, *new_val);
    } else {
      pots.insert(*pos, '.');
    }
  }
}

fn read_input(filename: &str) -> (BTreeMap<i32, char>, BTreeMap<String, char>) {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");

  let mut pots = BTreeMap::new();
  let mut patterns = BTreeMap::new();

  let mut rows: Vec<String> = content
    .split_terminator('\n')
    .map(|s| s.to_string())
    .collect();
  let initial_state_str = rows.remove(0);
  let initial_state: Vec<&str> =
    initial_state_str.split_terminator(": ").collect();
  rows.remove(0);
  let mut chars = initial_state[1].chars();
  let mut i = 0;
  while let Some(ch) = chars.next() {
    pots.insert(i, ch);
    i += 1;
  }

  let re = Regex::new(r"([\.\#]+)\s=>\s([\.\#])").unwrap();
  for s in rows {
    let caps = re.captures(&s).unwrap();
    patterns.insert(caps[1].to_string(), caps[2].chars().next().unwrap());
  }

  (pots, patterns)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let (pots, patterns) = read_input("test-input.txt");
    assert_eq!(find_sum_after_20th_gen(&pots, &patterns), 325);
  }
}
