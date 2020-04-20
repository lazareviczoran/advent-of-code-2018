#![feature(drain_filter)]
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let combat_arena = read_input("input.txt");

  println!(
    "Day 15: Beverage Bandits part1 solution\n{:?}",
    &mut combat_arena.clone().simulate_combat()
  );

  println!(
    "Day 15: Beverage Bandits part2 solution\n{:?}",
    find_outcome_when_elves_win_without_dying(&combat_arena)
  );
}

fn find_outcome_when_elves_win_without_dying(arena: &CombatArena) -> usize {
  let elves_count = arena.count_elves();
  search(arena, 4, 4, elves_count, &mut HashMap::new())
}

fn search(
  arena: &CombatArena,
  low: usize,
  high: usize,
  elves_count: usize,
  computed: &mut HashMap<usize, (usize, usize)>,
) -> usize {
  let mid = (low + high + 1) / 2;
  let low_count;
  if let Some((_l_score, l_count)) = computed.get(&low) {
    low_count = *l_count;
  } else {
    let mut low_arena = arena.clone();
    low_arena.elf_attack_power = low as i32;
    let low_score = low_arena.simulate_combat();
    low_count = low_arena.count_elves();
    computed.insert(low, (low_score, low_count));
  }

  let high_score;
  let high_count;
  if let Some((h_score, h_count)) = computed.get(&high) {
    high_score = *h_score;
    high_count = *h_count;
  } else {
    let mut high_arena = arena.clone();
    high_arena.elf_attack_power = high as i32;
    high_score = high_arena.simulate_combat();
    high_count = high_arena.count_elves();
    computed.insert(high, (high_score, high_count));
  }

  if high == low {
    if elves_count == high_count {
      return high_score;
    }
    return search(arena, high, high * 2, elves_count, computed);
  } else if high_count < elves_count {
    return search(arena, high, high * 2, elves_count, computed);
  } else if low_count < elves_count {
    return search(arena, mid, high, elves_count, computed);
  } else {
    return search(arena, low - (high - low) / 2, low, elves_count, computed);
  }
}

#[derive(Debug, Clone)]
struct CombatArena {
  rounds: usize,
  map: Vec<Vec<char>>,
  units: Vec<Unit>,
  elf_attack_power: i32,
}
impl CombatArena {
  pub fn new(map: Vec<Vec<char>>, units: Vec<Unit>) -> Self {
    Self {
      map,
      units,
      rounds: 0,
      elf_attack_power: 3,
    }
  }

  pub fn simulate_combat(&mut self) -> usize {
    loop {
      if !self.can_proceed_combat() {
        break;
      }

      self.run_next_round();
    }
    let health_sum =
      self.units.iter().fold(0, |acc, u| acc + u.health as usize);
    self.rounds * health_sum
  }

  pub fn run_next_round(&mut self) {
    let mut i = 0;
    while i < self.units.len() {
      let mut unit = self.units.remove(i);
      unit.move_and_battle(self);
      self.units.insert(i, unit);
      i += 1;
      if !self.can_proceed_combat() {
        break;
      }
    }
    if i == self.units.len() {
      self.rounds += 1;
    }
    self.units.drain_filter(|u| u.health <= 0);
    self.units.sort();
  }

  pub fn can_proceed_combat(&self) -> bool {
    let unit_type = self.units[0].unit_type;
    let mut has_one_alive_type = true;
    for unit in self.units.iter().skip(1) {
      if unit.is_alive() {
        has_one_alive_type = has_one_alive_type && unit_type == unit.unit_type;
      }
      if !has_one_alive_type {
        break;
      }
    }
    !has_one_alive_type
  }

  pub fn count_elves(&self) -> usize {
    self.units.iter().fold(
      0,
      |acc, u| {
        if u.unit_type == 'E' {
          acc + 1
        } else {
          acc
        }
      },
    )
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Unit {
  unit_type: char,
  pos: (usize, usize),
  health: i32,
}
impl Unit {
  pub fn new(unit_type: char, pos: (usize, usize)) -> Self {
    Self {
      unit_type,
      pos,
      health: 200,
    }
  }

  pub fn is_alive(&self) -> bool {
    self.health > 0
  }

  pub fn move_and_battle(&mut self, combat_arena: &mut CombatArena) {
    let (x, y) = self.pos;
    combat_arena.map[x][y] = '.';
    if !self.is_alive() {
      return;
    }

    let unit_type = self.unit_type;
    let opponent_type;
    if unit_type == 'E' {
      opponent_type = 'G';
    } else {
      opponent_type = 'E';
    }

    // find closest opponents
    let next_moves = self.find_closest_opponents(&combat_arena, opponent_type);
    // move if necessary
    if next_moves.len() > 0 {
      self.pos = next_moves[0];
    }

    // battle if possible

    let candidates = self.get_opponent_candidates(combat_arena);
    if candidates.len() > 0 {
      for i in 0..combat_arena.units.len() {
        if combat_arena.units[i] == candidates[0] {
          if self.unit_type == 'E' {
            combat_arena.units[i].health -= combat_arena.elf_attack_power;
          } else {
            combat_arena.units[i].health -= 3;
          }
          if !combat_arena.units[i].is_alive() {
            let (opp_x, opp_y) = combat_arena.units[i].pos;
            combat_arena.map[opp_x][opp_y] = '.';
          }
        }
      }
    }

    let (x, y) = self.pos;
    combat_arena.map[x][y] = self.unit_type;
  }

  pub fn get_opponent_candidates(
    &self,
    combat_arena: &CombatArena,
  ) -> Vec<Unit> {
    let mut candidates = Vec::new();
    let (x, y) = self.pos;
    for unit in combat_arena.units.iter() {
      if (unit.pos == (x - 1, y)
        || unit.pos == (x + 1, y)
        || unit.pos == (x, y - 1)
        || unit.pos == (x, y + 1))
        && unit.is_alive()
        && unit.unit_type != self.unit_type
      {
        candidates.push(*unit);
      }
    }
    candidates.sort_by(|a, b| a.health.cmp(&b.health).then(a.cmp(&b)));
    candidates
  }

  pub fn find_closest_opponents(
    &self,
    combat_arena: &CombatArena,
    opp_char: char,
  ) -> Vec<(usize, usize)> {
    let (x, y) = self.pos;
    let mut paths: Vec<(usize, usize)> = Vec::new();
    let mut has_alive_opps = false;
    for unit in combat_arena.units.iter() {
      if unit.unit_type == opp_char && unit.is_alive() {
        has_alive_opps = true;
        break;
      }
    }
    if has_alive_opps {
      let mut queue = vec![(x, y, 0, None)];
      let mut visited = HashMap::new();
      let mut min_steps = usize::max_value();
      while !queue.is_empty() {
        let (curr_x, curr_y, steps, mut next_move) = queue.remove(0);
        if visited.get(&(curr_x, curr_y)).is_some() {
          continue;
        }

        if steps > min_steps {
          break;
        }
        if next_move.is_none() && (curr_x, curr_y) != (x, y) {
          next_move = Some((curr_x, curr_y));
        }

        visited.insert((curr_x, curr_y), steps);
        if combat_arena.map[curr_x][curr_y] == opp_char && steps <= min_steps {
          min_steps = steps;
          if let Some(unit_move) = next_move {
            if unit_move != (curr_x, curr_y) && !paths.contains(&unit_move) {
              paths.push(unit_move);
              break;
            }
          }
        } else if combat_arena.map[curr_x][curr_y] == '.' {
          queue.push((curr_x, curr_y - 1, steps + 1, next_move));
          queue.push((curr_x - 1, curr_y, steps + 1, next_move));
          queue.push((curr_x + 1, curr_y, steps + 1, next_move));
          queue.push((curr_x, curr_y + 1, steps + 1, next_move));
        }
      }
    }
    paths
  }
}

impl PartialOrd for Unit {
  fn partial_cmp(&self, other: &Unit) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl Ord for Unit {
  fn cmp(&self, other: &Unit) -> Ordering {
    let (x, y) = self.pos;
    let (other_x, other_y) = other.pos;
    y.cmp(&other_y).then(x.cmp(&other_x))
  }
}

fn read_input(filename: &str) -> CombatArena {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");

  let mut map = Vec::new();
  let mut units = Vec::new();

  let mut y = 0;
  for s in content.split_terminator('\n') {
    let mut x = 0;
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
      if ch == 'G' || ch == 'E' {
        units.push(Unit::new(ch, (x, y)));
      }
      if map.len() == x {
        map.push(vec![ch]);
      } else {
        map[x].push(ch);
      }
      x += 1;
    }
    y += 1;
  }

  CombatArena::new(map, units)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn find_closest_test1() {
    let mut combat_arena = read_input("closest-test.txt");

    let unit = combat_arena.units.remove(0);
    let (x, y) = unit.pos;
    combat_arena.map[x][y] = '.';

    assert_eq!(
      unit.find_closest_opponents(&combat_arena, 'G'),
      vec![(3, 1)]
    );
  }

  #[test]
  fn find_closest_test2() {
    let mut combat_arena = read_input("test-input.txt");

    let unit = combat_arena.units.remove(1);
    let (x, y) = unit.pos;
    combat_arena.map[x][y] = '.';

    assert_eq!(unit.find_closest_opponents(&combat_arena, 'G'), vec![]);
  }

  #[test]
  fn find_closest_test3() {
    let mut combat_arena = read_input("closest-test2.txt");

    let unit = combat_arena.units.remove(3);
    let (x, y) = unit.pos;
    combat_arena.map[x][y] = '.';

    assert_eq!(
      unit.find_closest_opponents(&combat_arena, 'E'),
      vec![(3, 1)]
    );
  }

  #[test]
  fn part1_test1() {
    let mut combat_arena = read_input("test-input.txt");

    assert_eq!(combat_arena.simulate_combat(), 27730);
  }

  #[test]
  fn part1_test2() {
    let mut combat_arena = read_input("test-input2.txt");

    assert_eq!(combat_arena.simulate_combat(), 36334);
  }

  #[test]
  fn part1_test3() {
    let mut combat_arena = read_input("test-input3.txt");

    assert_eq!(combat_arena.simulate_combat(), 39514);
  }

  #[test]
  fn part1_test4() {
    let mut combat_arena = read_input("test-input4.txt");

    assert_eq!(combat_arena.simulate_combat(), 27755);
  }

  #[test]
  fn part1_test5() {
    let mut combat_arena = read_input("test-input5.txt");

    assert_eq!(combat_arena.simulate_combat(), 28944);
  }

  #[test]
  fn part1_test6() {
    let mut combat_arena = read_input("test-input6.txt");

    assert_eq!(combat_arena.simulate_combat(), 18740);
  }

  #[test]
  fn part2_test1() {
    let combat_arena = read_input("test-input.txt");

    assert_eq!(
      find_outcome_when_elves_win_without_dying(&combat_arena),
      4988
    );
  }

  #[test]
  fn part2_test2() {
    let combat_arena = read_input("test-input3.txt");

    assert_eq!(
      find_outcome_when_elves_win_without_dying(&combat_arena),
      31284
    );
  }

  #[test]
  fn part2_test3() {
    let combat_arena = read_input("test-input4.txt");

    assert_eq!(
      find_outcome_when_elves_win_without_dying(&combat_arena),
      3478
    );
  }

  #[test]
  fn part2_test4() {
    let combat_arena = read_input("test-input5.txt");

    assert_eq!(
      find_outcome_when_elves_win_without_dying(&combat_arena),
      6474
    );
  }

  #[test]
  fn part2_test5() {
    let combat_arena = read_input("test-input6.txt");

    assert_eq!(
      find_outcome_when_elves_win_without_dying(&combat_arena),
      1140
    );
  }
}
