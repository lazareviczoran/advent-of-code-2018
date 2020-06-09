use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let map = read_input("input.txt");
  println!(
    "Day 18: Settlers of The North Pole part1 solution\n{:?}",
    calculate_total_resource_value(&map, 10)
  );

  println!(
    "Day 18: Settlers of The North Pole part2 solution\n{:?}",
    calculate_total_resource_value(&map, 1_000_000_000)
  );
}

fn calculate_total_resource_value(
  map: &Vec<Vec<char>>,
  iterations: usize,
) -> usize {
  let mut previous_states = HashMap::new();
  previous_states.insert(map.clone(), 0);
  let mut curr_map = map.clone();
  let mut minutes = 1;
  let mut repeat_min = 0;
  while minutes <= iterations {
    curr_map = apply_transformation(&curr_map);
    if let Some(rep_min) = previous_states.get(&curr_map) {
      repeat_min = *rep_min;
      break;
    }
    previous_states.insert(curr_map.clone(), minutes);
    minutes += 1;
  }

  if minutes < iterations {
    let cycle = minutes - repeat_min;
    let remaining = (iterations - minutes) % cycle;
    for (m, p) in previous_states.iter() {
      if *p == repeat_min + remaining {
        curr_map = m.clone();
        break;
      }
    }
  }

  let mut count = (0, 0, 0);
  for j in 0..curr_map[0].len() {
    for i in 0..curr_map.len() {
      match curr_map[i][j] {
        '.' => count.0 += 1,
        '|' => count.1 += 1,
        '#' => count.2 += 1,
        _ => panic!("unexpected char {}", map[i][j]),
      }
    }
  }

  count.1 * count.2
}

fn apply_transformation(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut res = map.clone();
  for j in 0..res[0].len() {
    for i in 0..res.len() {
      res[i][j] = calculate_field_value(map, i, j);
    }
  }

  res
}

fn calculate_field_value(map: &Vec<Vec<char>>, i: usize, j: usize) -> char {
  let prev_val = map[i][j];
  let from_i = if i > 0 { i - 1 } else { i };
  let to_i = if i < map.len() - 1 { i + 1 } else { i };
  let from_j = if j > 0 { j - 1 } else { j };
  let to_j = if j < map[0].len() - 1 { j + 1 } else { j };
  let mut count = (0, 0, 0);
  for y in from_j..=to_j {
    for x in from_i..=to_i {
      if x == i && y == j {
        continue;
      }
      match map[x][y] {
        '.' => count.0 += 1,
        '|' => count.1 += 1,
        '#' => count.2 += 1,
        _ => panic!("unexpected char {}", map[x][y]),
      }
    }
  }
  let mut new_val = prev_val;
  if prev_val == '.' && count.1 >= 3 {
    new_val = '|';
  } else if prev_val == '|' && count.2 >= 3 {
    new_val = '#';
  } else if prev_val == '#' && !(count.1 >= 1 && count.2 >= 1) {
    new_val = '.';
  }

  new_val
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");
  let rows = content.split_terminator('\n').collect::<Vec<&str>>();
  let mut map = vec![vec![' '; rows.len()]; rows[0].len()];
  let mut j = 0;
  for r in rows {
    let mut i = 0;
    let mut chars = r.chars();
    while let Some(ch) = chars.next() {
      map[i][j] = ch;
      i += 1;
    }
    j += 1;
  }
  map
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let map = read_input("test-input.txt");
    assert_eq!(calculate_total_resource_value(&map, 10), 1147);
  }

  #[test]
  fn part2_test() {
    let map = read_input("test-input.txt");
    assert_eq!(calculate_total_resource_value(&map, 1_000_000_000), 1147);
  }
}
