use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let sleep_time_intervals = read_input("input.txt");
  println!(
    "Day 4: Repose Record part1 solution\n {}",
    find_strategy1_solution(&sleep_time_intervals)
  );

  println!(
    "Day 4: Repose Record part2 solution\n {}",
    find_strategy2_solution(&sleep_time_intervals)
  );
}

fn find_strategy2_solution(
  sleep_time_intervals: &HashMap<usize, Vec<(usize, usize)>>,
) -> usize {
  let mut guard_id = 0;
  let mut current_max = 0;
  let mut most_overslept_minute = 0;
  for (id, intervals) in sleep_time_intervals {
    let (minute, sleep_count) = find_most_frequent_minute_for_sleep(intervals);
    if sleep_count > current_max {
      current_max = sleep_count;
      guard_id = *id;
      most_overslept_minute = minute;
    }
  }

  guard_id * most_overslept_minute
}

fn find_strategy1_solution(
  sleep_time_intervals: &HashMap<usize, Vec<(usize, usize)>>,
) -> usize {
  let mut guard_id = 0;
  let mut current_max = 0;
  for (id, intervals) in sleep_time_intervals.clone().iter_mut() {
    let sum = intervals
      .iter_mut()
      .fold(0, |acc, interval| acc + interval.1 - interval.0);
    if sum > current_max {
      guard_id = *id;
      current_max = sum;
    }
  }

  let (minute, _) = find_most_frequent_minute_for_sleep(
    &sleep_time_intervals.get(&guard_id).unwrap(),
  );

  guard_id * minute
}

fn find_most_frequent_minute_for_sleep(
  intervals: &Vec<(usize, usize)>,
) -> (usize, usize) {
  let mut sleep_count = vec![0; 60];
  let mut most_overslept_minute = 0;
  for interval in intervals {
    for i in interval.0..=interval.1 {
      sleep_count[i] += 1;
      if sleep_count[most_overslept_minute] < sleep_count[i] {
        most_overslept_minute = i;
      }
    }
  }
  (most_overslept_minute, sleep_count[most_overslept_minute])
}

fn read_input(filename: &str) -> HashMap<usize, Vec<(usize, usize)>> {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");

  let mut items: Vec<String> = content
    .split_terminator('\n')
    .map(|a| a.to_string())
    .collect();
  items.sort();
  let time_re = Regex::new(r"\[\d+\-\d+\-\d+\s\d+:(\d+)\]").unwrap();
  let guard_id_re = Regex::new(r"Guard\s\#(\d+)").unwrap();

  let mut sleep_time_intervals: HashMap<usize, Vec<(usize, usize)>> =
    HashMap::new();
  let mut iter = items.iter().peekable();
  while let Some(guard) = iter.next() {
    let guard_caps = guard_id_re.captures(guard).unwrap();
    let guard_id = guard_caps[1].parse::<usize>().unwrap();
    if let Some(next_line) = iter.peek() {
      if next_line.contains("Guard") {
        continue;
      }
    }

    let mut next_line = iter.peek();
    while next_line.is_some() && !next_line.unwrap().contains("Guard") {
      let from_date_time_str = iter.next().unwrap();
      let to_date_time_str = iter.next().unwrap();

      let from_caps = time_re.captures(from_date_time_str).unwrap();
      let from = from_caps[1].parse::<usize>().unwrap();

      let to_caps = time_re.captures(to_date_time_str).unwrap();
      // subtracting 1 because at the stated time
      // the guard is considered to be awake
      let to = to_caps[1].parse::<usize>().unwrap() - 1;
      if let Some(guard_sleep_intervals) =
        sleep_time_intervals.get_mut(&guard_id)
      {
        guard_sleep_intervals.push((from, to));
      } else {
        sleep_time_intervals.insert(guard_id, vec![(from, to)]);
      }
      next_line = iter.peek();
    }
  }

  sleep_time_intervals
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let sleep_time_intervals = read_input("test-input.txt");
    assert_eq!(find_strategy1_solution(&sleep_time_intervals), 240);
  }

  #[test]
  fn part2_test() {
    let sleep_time_intervals = read_input("test-input.txt");
    assert_eq!(find_strategy2_solution(&sleep_time_intervals), 4455);
  }
}
