use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn main() {
  // input
  // depth: 4848
  // target: 15,700

  let depth = 4848;
  let target = (15, 700);
  let mut map = build_map(depth, &target);
  println!(
    "Day 22: Mode Maze part1 solution\n{:?}",
    calculate_risk_level(&map, &target)
  );

  println!(
    "Day 22: Mode Maze part2 solution\n{:?}",
    find_fastest_way(&mut map, depth, &target)
  );
}

fn find_fastest_way(
  map: &mut HashMap<(i32, i32), FieldInfo>,
  depth: i32,
  target: &(i32, i32),
) -> i32 {
  let mut heap = BinaryHeap::new();
  heap.push(State::new((0, 0), Tool::Torch, 0));
  let mut visited = HashMap::new();
  while let Some(State { time, pos, tool }) = heap.pop() {
    let field_type;
    if let Some(info) = map.get(&pos) {
      field_type = info.r#type;
    } else {
      let field_info = FieldInfo::new(map, target, pos, depth);
      field_type = field_info.r#type;
    }
    if !get_eligible_tools(field_type).contains(&tool) {
      continue;
    }

    if let Some(visited_minutes) = visited.get(&(pos, tool)) {
      if *visited_minutes <= time {
        continue;
      }
    }
    visited.insert((pos, tool), time);
    if &pos == target && tool == Tool::Torch {
      return time;
    }

    for (dx, dy) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
      let new_x = pos.0 + dx;
      let new_y = pos.1 + dy;
      if new_x >= 0 && new_y >= 0 {
        heap.push(State::new((new_x, new_y), tool, time + 1));
      }
    }
    for t in vec![Tool::ClimbingGear, Tool::Torch, Tool::Neither] {
      if tool != t {
        heap.push(State::new(pos, t, time + 7));
      }
    }
  }
  unreachable!()
}

fn get_eligible_tools(field_type: char) -> Vec<Tool> {
  match field_type {
    '.' => vec![Tool::ClimbingGear, Tool::Torch],
    '=' => vec![Tool::ClimbingGear, Tool::Neither],
    _ => vec![Tool::Torch, Tool::Neither],
  }
}

fn calculate_risk_level(
  map: &HashMap<(i32, i32), FieldInfo>,
  target: &(i32, i32),
) -> i32 {
  let mut risk_level = 0;
  for y in 0..=target.1 {
    for x in 0..=target.0 {
      let field_info = map.get(&(x, y)).unwrap();
      risk_level += match field_info.r#type {
        '.' => 0,
        '=' => 1,
        _ => 2,
      }
    }
  }
  risk_level
}

fn build_map(
  depth: i32,
  target: &(i32, i32),
) -> HashMap<(i32, i32), FieldInfo> {
  let mut map = HashMap::new();
  FieldInfo::new(&mut map, target, (0, 0), depth);
  for y in 0..=target.1 {
    for x in 0..=target.0 {
      FieldInfo::new(&mut map, target, (x, y), depth);
    }
  }
  map
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
  time: i32,
  pos: (i32, i32),
  tool: Tool,
}
impl State {
  pub fn new(pos: (i32, i32), tool: Tool, time: i32) -> Self {
    Self { pos, time, tool }
  }
}
impl Ord for State {
  fn cmp(&self, other: &State) -> Ordering {
    (other.time).cmp(&(self.time))
  }
}
impl PartialOrd for State {
  fn partial_cmp(&self, other: &State) -> Option<Ordering> {
    Some(self.cmp(&other))
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Tool {
  Torch,
  ClimbingGear,
  Neither,
}

#[derive(Debug, Clone, Copy)]
struct FieldInfo {
  erosion_level: i32,
  r#type: char,
}
impl FieldInfo {
  pub fn new(
    map: &mut HashMap<(i32, i32), FieldInfo>,
    target: &(i32, i32),
    pos: (i32, i32),
    depth: i32,
  ) -> Self {
    let geo_index = if pos == *target || pos == (0, 0) {
      0
    } else if pos.1 == 0 {
      pos.0 * 16807
    } else if pos.0 == 0 {
      pos.1 * 48271
    } else {
      let val1;
      if let Some(field) = map.get(&(pos.0 - 1, pos.1)) {
        val1 = field.erosion_level;
      } else {
        val1 =
          FieldInfo::new(map, target, (pos.0 - 1, pos.1), depth).erosion_level;
      }
      let val2;
      if let Some(field) = map.get(&(pos.0, pos.1 - 1)) {
        val2 = field.erosion_level;
      } else {
        val2 =
          FieldInfo::new(map, target, (pos.0, pos.1 - 1), depth).erosion_level;
      }
      val1 * val2
    };
    let erosion_level = (geo_index + depth) % 20183;
    let r#type = match erosion_level % 3 {
      0 => '.',
      1 => '=',
      _ => '|',
    };
    let field = Self {
      erosion_level,
      r#type,
    };
    map.insert(pos, field);

    field
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_test() {
    let depth = 510;
    let target = (10, 10);
    let map = build_map(depth, &target);
    assert_eq!(calculate_risk_level(&map, &target), 114)
  }

  #[test]
  fn part2_test() {
    let depth = 510;
    let target = (10, 10);
    let mut map = build_map(depth, &target);
    assert_eq!(find_fastest_way(&mut map, depth, &target), 45)
  }
}
