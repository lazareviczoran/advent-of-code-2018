use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let box_ids = read_input("input.txt");
  println!(
    "Day 2: Inventory Management System part1 solution\n {}",
    calculate_list_checksum(&box_ids)
  );

  println!(
    "Day 2: Inventory Management System part2 solution\n {}",
    get_common_letters(&box_ids)
  );
}

fn calculate_list_checksum(ids_vec: &Vec<String>) -> i32 {
  let mut count_exactly_two = 0;
  let mut count_exactly_three = 0;
  for id in ids_vec {
    let mut appearances_map = HashMap::new();
    let chars = id.chars();
    for ch in chars {
      if let Some(count) = appearances_map.get_mut(&ch) {
        *count += 1;
      } else {
        appearances_map.insert(ch, 1);
      }
    }

    let appearances: Vec<i32> = appearances_map.values().cloned().collect();
    if appearances.contains(&2) {
      count_exactly_two += 1;
    }
    if appearances.contains(&3) {
      count_exactly_three += 1;
    }
  }
  count_exactly_two * count_exactly_three
}

fn get_common_letters(ids_vec: &Vec<String>) -> String {
  let id_len = ids_vec[0].len();
  let mut common_letters = String::new();
  for i in 0..ids_vec.len() {
    for j in i + 1..ids_vec.len() {
      // find common letters between 2 ids
      let mut curr_common = String::new();
      for n in 0..id_len {
        let curr_1 = ids_vec[i].get(n..n + 1).unwrap();
        let curr_2 = ids_vec[j].get(n..n + 1).unwrap();
        if curr_1 == curr_2 {
          curr_common.push_str(curr_1);
        }
      }
      if curr_common.len() > common_letters.len() {
        common_letters = curr_common;
      }
    }
  }
  common_letters
}

fn read_input(filename: &str) -> Vec<String> {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");
  content
    .split_terminator('\n')
    .map(|s| s.to_string())
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let ids = vec![
      String::from("abcdef"),
      String::from("bababc"),
      String::from("abbcde"),
      String::from("abcccd"),
      String::from("aabcdd"),
      String::from("abcdee"),
      String::from("ababab"),
    ];
    assert_eq!(calculate_list_checksum(&ids), 12);
  }

  #[test]
  fn part2_test() {
    let ids = vec![
      String::from("abcde"),
      String::from("fghij"),
      String::from("klmno"),
      String::from("pqrst"),
      String::from("fguij"),
      String::from("axcye"),
      String::from("wvxyz"),
    ];
    assert_eq!(get_common_letters(&ids), "fgij");
  }
}
