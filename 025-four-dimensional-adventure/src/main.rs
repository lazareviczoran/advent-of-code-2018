use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let points = read_input("input.txt");
  println!(
    "Day 25: Four-Dimensional Adventure solution\n{}",
    find_constellation_count(&points)
  );
}

fn find_constellation_count(points: &Vec<Vec<i32>>) -> i32 {
  let mut graph = HashMap::new();
  let mut remaining = HashSet::new();
  for p1 in 0..points.len() {
    remaining.insert(p1);
    for p2 in 0..points.len() {
      if p1 != p2 {
        let distance = points[p1]
          .iter()
          .zip(points[p2].iter())
          .fold(0, |acc, (x, y)| acc + (x - y).abs());
        if distance <= 3 {
          let neighbours = graph.entry(p1).or_insert(Vec::new());
          neighbours.push(p2);
        }
      }
    }
  }

  let mut queue = vec![0];
  let mut constellation_count = 0;
  while !queue.is_empty() {
    let curr = queue.remove(0);
    remaining.remove(&curr);

    // add connections
    if let Some(neighbours) = graph.get(&curr) {
      for n in neighbours {
        if remaining.contains(n) {
          queue.push(*n);
        }
      }
    }

    if queue.is_empty() {
      constellation_count += 1;
      if !remaining.is_empty() {
        queue.push(*remaining.iter().next().unwrap());
      }
    }
  }

  constellation_count
}

fn read_input(filename: &str) -> Vec<Vec<i32>> {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");

  let mut points = Vec::new();
  for s in content.split_terminator('\n') {
    let point = s
      .split_terminator(',')
      .map(|x| x.trim().parse::<i32>().unwrap())
      .collect();
    points.push(point);
  }
  points
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_test1() {
    let points = read_input("test-input.txt");
    assert_eq!(find_constellation_count(&points), 2)
  }

  #[test]
  fn part1_test2() {
    let points = read_input("test-input2.txt");
    assert_eq!(find_constellation_count(&points), 4)
  }

  #[test]
  fn part1_test3() {
    let points = read_input("test-input3.txt");
    assert_eq!(find_constellation_count(&points), 3)
  }

  #[test]
  fn part1_test4() {
    let points = read_input("test-input4.txt");
    assert_eq!(find_constellation_count(&points), 8)
  }
}
