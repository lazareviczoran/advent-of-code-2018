#![feature(map_first_last, drain_filter)]
use regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let items = read_input("input.txt");
  println!(
    "Day 7: The Sum of Its Parts part1 solution\n {}",
    find_instruction_order(&items)
  );

  println!(
    "Day 7: The Sum of Its Parts part2 solution\n {}",
    find_instruction_order_with_workers(&items, 5, 60).1
  );
}

fn find_instruction_order_with_workers(
  items: &HashMap<char, Item>,
  workers_limit: usize,
  step_time_base: usize,
) -> (String, usize) {
  let mut res = String::new();
  let mut time_spent = 0;
  let mut active_workers: Vec<Task> = Vec::new();

  let mut tree_set = BTreeSet::new();
  for (id, item) in items.iter() {
    if item.required_by.is_empty() {
      tree_set.insert(*id);
    }
  }

  while items.len() != res.len() || res.len() == 0 {
    active_workers.drain_filter(|t| t.remaining_time == 0);
    while tree_set.len() > 0 && active_workers.len() < workers_limit {
      let next_id = tree_set.pop_first().unwrap();
      let task = Task::new(next_id, step_time_base);
      active_workers.push(task);
    }

    for task in active_workers.iter_mut() {
      task.remaining_time -= 1;
      if task.remaining_time == 0 {
        res.push(task.item);
        let next_item = items.get(&task.item).unwrap();
        for i in next_item.links.iter() {
          let target_item = items.get(i).unwrap();
          if !tree_set.contains(i)
            && contains_all(&res, &target_item.required_by)
          {
            tree_set.insert(*i);
          }
        }
      }
    }
    time_spent += 1;
  }

  (res, time_spent)
}

fn find_instruction_order(items: &HashMap<char, Item>) -> String {
  let mut res = String::new();
  let mut tree_set = BTreeSet::new();
  for (id, item) in items.iter() {
    if item.required_by.is_empty() {
      tree_set.insert(*id);
    }
  }

  while !tree_set.is_empty() {
    let next_id = tree_set.pop_first().unwrap();
    res.push(next_id);
    let next_item = items.get(&next_id).unwrap();
    for i in next_item.links.iter() {
      let target_item = items.get(i).unwrap();
      if !tree_set.contains(i) && contains_all(&res, &target_item.required_by) {
        tree_set.insert(*i);
      }
    }
  }

  res
}

fn contains_all(curr: &str, items: &HashSet<char>) -> bool {
  let mut has_all_required = true;
  for i in items {
    if !has_all_required {
      return false;
    }
    has_all_required = has_all_required && curr.contains(*i);
  }
  has_all_required
}

fn read_input(filename: &str) -> HashMap<char, Item> {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");
  let re = Regex::new(
    r"Step ([A-Z]) must be finished before step ([A-Z]) can begin\.",
  )
  .unwrap();

  let mut items: HashMap<char, Item> = HashMap::new();
  for s in content.split_terminator('\n') {
    let caps = re.captures(s).unwrap();
    let from = &caps[1].chars().next().unwrap();
    let to = &caps[2].chars().next().unwrap();
    if let Some(item) = items.get_mut(from) {
      item.links.insert(*to);
    } else {
      let mut item = Item::new();
      item.links.insert(*to);
      items.insert(*from, item);
    }
    if let Some(item) = items.get_mut(to) {
      item.required_by.insert(*from);
    } else {
      let mut item = Item::new();
      item.required_by.insert(*from);
      items.insert(*to, item);
    }
  }

  items
}

#[derive(Debug)]
struct Task {
  item: char,
  remaining_time: usize,
}
impl Task {
  pub fn new(item: char, step_time_base: usize) -> Task {
    Task {
      item,
      remaining_time: item as u8 as usize - 'A' as u8 as usize
        + 1
        + step_time_base,
    }
  }
}

#[derive(Debug)]
struct Item {
  links: HashSet<char>,
  required_by: HashSet<char>,
}
impl Item {
  pub fn new() -> Item {
    Item {
      links: HashSet::new(),
      required_by: HashSet::new(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let items = read_input("test-input.txt");
    assert_eq!(find_instruction_order(&items), "CABDFE");
  }

  #[test]
  fn part2_test() {
    let items = read_input("test-input.txt");
    let (order, time) = find_instruction_order_with_workers(&items, 2, 0);
    assert_eq!(order, "CABFDE");
    assert_eq!(time, 15);
  }
}
