use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let combat = read_input("input.txt");
  println!(
    "Day 24: Immune System Simulator 20XX part1 solution\n{:?}",
    &mut combat.clone().fight(0).0
  );

  println!(
    "Day 24: Immune System Simulator 20XX part2 solution\n{:?}",
    find_unit_count_in_win_with_smallest_boost(&combat)
  );
}

fn find_unit_count_in_win_with_smallest_boost(combat: &Combat) -> i32 {
  let mut prev_boost = 0;
  let mut step = 1000;
  let mut prev_outcome = (22859, GroupType::Infection);
  loop {
    let mut curr_combat = combat.clone();
    let boost = prev_boost + step;
    let curr_outcome = curr_combat.fight(boost);
    if step == 0 || curr_outcome.1 != prev_outcome.1 && step.abs() == 1 {
      if curr_outcome.1 == GroupType::ImmuneSystem {
        return curr_outcome.0;
      } else {
        return prev_outcome.0;
      };
    }

    if curr_outcome.1 != prev_outcome.1 {
      step = -step / 2;
    }
    prev_outcome = curr_outcome;
    prev_boost = boost;
  }
}

#[derive(Debug, Clone)]
struct Combat {
  immune_system: Vec<Group>,
  infections: Vec<Group>,
}
impl Combat {
  pub fn new(immune_system: Vec<Group>, infections: Vec<Group>) -> Self {
    Self {
      immune_system,
      infections,
    }
  }

  pub fn fight(&mut self, boost: i32) -> (i32, GroupType) {
    if boost > 0 {
      for item in self.immune_system.iter_mut() {
        item.attack_damage += boost;
      }
    }
    while !self.immune_system.is_empty() && !self.infections.is_empty() {
      // target selection
      self.immune_system.sort();
      self.infections.sort();
      for is in self.immune_system.iter_mut() {
        is.target = find_target(is, &mut self.infections);
      }
      for inf in self.infections.iter_mut() {
        inf.target = find_target(inf, &mut self.immune_system);
      }

      // attacking phase
      let mut combined = Vec::new();
      combined.append(&mut self.immune_system);
      combined.append(&mut self.infections);
      combined.sort_by(|a, b| b.initiative.cmp(&a.initiative));
      let mut total_kills = 0;
      for i in 0..combined.len() {
        if combined[i].units > 0 && combined[i].target.is_some() {
          let target_initiative = combined[i].target.unwrap();
          let target_index = combined
            .iter()
            .position(|a| a.initiative == target_initiative)
            .unwrap();
          let target = combined[target_index].clone();
          let damage = combined[i].calculate_damage(&target);
          let killed_units = damage / combined[target_index].hit_points;
          combined[target_index].units -= killed_units;
          total_kills += killed_units;
        }
      }

      // remove killed groups
      for item in combined.iter_mut() {
        if item.units > 0 {
          item.taken = false;
          if item.group_type == GroupType::ImmuneSystem {
            self.immune_system.push(item.clone());
          } else {
            self.infections.push(item.clone());
          }
        }
      }
      if total_kills == 0 {
        break;
      }
    }

    if !self.infections.is_empty() {
      (
        self.infections.iter().fold(0, |acc, g| acc + g.units),
        GroupType::Infection,
      )
    } else {
      (
        self.immune_system.iter().fold(0, |acc, g| acc + g.units),
        GroupType::ImmuneSystem,
      )
    }
  }
}

fn find_target(
  group: &mut Group,
  potential_targets: &mut Vec<Group>,
) -> Option<i32> {
  let mut candidates = Vec::new();
  for pt in potential_targets.iter() {
    let damage = group.calculate_damage(&pt);
    if !pt.taken && damage > 0 {
      candidates.push((damage, pt.effective_power(), pt.initiative));
    }
  }

  if candidates.is_empty() {
    None
  } else {
    candidates
      .sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)).then(b.2.cmp(&a.2)));
    let (_damage, _, initiative) = candidates[0];
    for item in potential_targets.iter_mut() {
      if item.initiative == initiative {
        item.taken = true;
        break;
      }
    }
    Some(initiative)
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Group {
  units: i32,
  hit_points: i32,
  group_type: GroupType,
  attack_damage: i32,
  attack_type: AttackType,
  initiative: i32,
  weaknesses: HashSet<AttackType>,
  immunities: HashSet<AttackType>,
  target: Option<i32>,
  taken: bool,
}
impl Group {
  pub fn new(
    units: i32,
    hit_points: i32,
    group_type: GroupType,
    attack_damage: i32,
    attack_type: AttackType,
    initiative: i32,
    weaknesses: HashSet<AttackType>,
    immunities: HashSet<AttackType>,
  ) -> Self {
    Self {
      units,
      hit_points,
      group_type,
      attack_damage,
      attack_type,
      initiative,
      weaknesses,
      immunities,
      target: None,
      taken: false,
    }
  }

  fn calculate_damage(&mut self, other: &Group) -> i32 {
    let mut damage = self.effective_power();
    if other.immunities.contains(&self.attack_type) {
      damage = 0;
    } else if other.weaknesses.contains(&self.attack_type) {
      damage *= 2;
    }
    damage
  }

  fn effective_power(&self) -> i32 {
    self.units * self.attack_damage
  }
}
impl Ord for Group {
  fn cmp(&self, other: &Self) -> Ordering {
    other
      .effective_power()
      .cmp(&self.effective_power())
      .then(other.initiative.cmp(&self.initiative))
  }
}
impl PartialOrd for Group {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(&other))
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GroupType {
  ImmuneSystem,
  Infection,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum AttackType {
  Cold,
  Radiation,
  Slashing,
  Fire,
  Bludgeoning,
}

fn get_attack_type(type_str: &str) -> AttackType {
  match type_str {
    "cold" => AttackType::Cold,
    "radiation" => AttackType::Radiation,
    "slashing" => AttackType::Slashing,
    "fire" => AttackType::Fire,
    "bludgeoning" => AttackType::Bludgeoning,
    _ => panic!("unexpected value {}", type_str),
  }
}

fn read_input(filename: &str) -> Combat {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");

  let mut immune_system = Vec::new();
  let mut infections = Vec::new();
  let group_contents = content.split_terminator("\n\n").collect::<Vec<&str>>();
  let re = Regex::new(r"(\d+) units each with (\d+) hit points (\(.*?\))?\s?with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)").unwrap();
  let mut group_type = GroupType::ImmuneSystem;
  for gc in group_contents {
    for s in gc.split_terminator('\n').skip(1) {
      let caps = re.captures(s).unwrap();
      let units = caps[1].parse::<i32>().unwrap();
      let hit_points = caps[2].parse::<i32>().unwrap();
      let mut weaknesses = HashSet::new();
      let mut immunities = HashSet::new();
      if let Some(match_str) = caps.get(3) {
        let w_and_i_strs = match_str.as_str();
        let strs = w_and_i_strs
          .get(1..w_and_i_strs.len() - 1)
          .unwrap()
          .split_terminator("; ")
          .collect::<Vec<&str>>();
        for st in strs {
          if st.starts_with("weak to ") {
            for type_str in st.split_at(8).1.split_terminator(", ") {
              weaknesses.insert(get_attack_type(type_str.trim()));
            }
          } else {
            for type_str in st.split_at(9).1.split_terminator(", ") {
              immunities.insert(get_attack_type(type_str.trim()));
            }
          }
        }
      }

      let attack_damage = caps[4].parse::<i32>().unwrap();
      let attack_type = get_attack_type(&caps[5]);
      let initiative = caps[6].parse::<i32>().unwrap();
      let group = Group::new(
        units,
        hit_points,
        group_type,
        attack_damage,
        attack_type,
        initiative,
        weaknesses,
        immunities,
      );
      if group_type == GroupType::ImmuneSystem {
        immune_system.push(group);
      } else {
        infections.push(group);
      }
    }
    group_type = GroupType::Infection;
  }

  Combat::new(immune_system, infections)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_test() {
    let mut combat = read_input("test-input.txt");
    assert_eq!(combat.fight(0), (5216, GroupType::Infection));
  }

  #[test]
  fn part2_test1() {
    let mut combat = read_input("test-input.txt");
    assert_eq!(combat.fight(1570), (51, GroupType::ImmuneSystem));
  }

  #[test]
  fn part2_test2() {
    let combat = read_input("test-input.txt");
    assert_eq!(find_unit_count_in_win_with_smallest_boost(&combat), 51);
  }
}
