use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut file = File::open("input.txt").expect("File not found");
  let mut polymer = String::new();
  file
    .read_to_string(&mut polymer)
    .expect("Failed to read input file");

  println!(
    "Day 5: Alchemical Reduction part1 solution\n {}",
    trigger_reactions(&polymer).len()
  );

  println!(
    "Day 5: Alchemical Reduction part2 solution\n {}",
    find_shortest_polymer_by_removing_one_item(&polymer).len()
  );
}

fn trigger_reactions(polymer: &str) -> String {
  let mut res = polymer.to_string();
  let mut i = 0;

  while i < res.len() {
    let prev_len = res.len();
    let curr_chunk = res.get(i..=i + 1);
    if let Some(chunk) = curr_chunk {
      let mut chars = chunk.chars();
      let curr_el = chars.next().unwrap();
      let next_el = chars.next().unwrap();
      if (curr_el.is_lowercase()
        && next_el.is_uppercase()
        && curr_el.to_uppercase().to_string() == next_el.to_string())
        || (curr_el.is_uppercase()
          && next_el.is_lowercase()
          && curr_el.to_lowercase().to_string() == next_el.to_string())
      {
        // the chunk contains elements that react, they should be destroyed
        res.remove(i);
        res.remove(i);
        if i > 0 {
          // moving index if possible to check if the previous element will
          // react with the new next element
          i -= 1;
        }
      }
    }
    if prev_len == res.len() {
      // nothing reacted, move index to the next element
      i += 1;
    }
  }

  res
}

fn find_shortest_polymer_by_removing_one_item(polymer: &str) -> String {
  let mut used_elements = HashSet::new();
  let mut curr_shortest_polymer = polymer.to_string();
  let mut chars = polymer.chars();
  while let Some(ch) = chars.next() {
    let lower = ch.to_lowercase().to_string();
    if used_elements.get(&lower).is_none() {
      let mut polymer_candidate = polymer.clone().to_string();
      polymer_candidate.retain(|c| c.to_lowercase().to_string() != lower);
      used_elements.insert(lower);
      let shortest_candidate = trigger_reactions(&polymer_candidate);
      if shortest_candidate.len() < curr_shortest_polymer.len() {
        curr_shortest_polymer = shortest_candidate;
      }
    }
  }

  curr_shortest_polymer
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let reduced_polymer = trigger_reactions("dabAcCaCBAcCcaDA");
    assert_eq!(reduced_polymer.len(), 10);
    assert_eq!(reduced_polymer, "dabCBAcaDA");
  }

  #[test]
  fn part2_test() {
    let shortest_polymer =
      find_shortest_polymer_by_removing_one_item("dabAcCaCBAcCcaDA");
    assert_eq!(shortest_polymer.len(), 4);
    assert_eq!(shortest_polymer, "daDA");
  }
}
