use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let claims = read_input("input.txt");
  println!(
    "Day 3: No Matter How You Slice It part1 solution\n {}",
    count_used_field_more_than_once(&claims, 1000)
  );

  println!(
    "Day 3: No Matter How You Slice It part2 solution\n {}",
    find_intact_claim(&claims, 1000)
  );
}

fn count_used_field_more_than_once(
  claims: &Vec<Claim>,
  field_size: usize,
) -> usize {
  let mut fields = init_fields(field_size, field_size);
  for claim in claims {
    for i in claim.position.0..claim.position.0 + claim.dimensions.0 {
      for j in claim.position.1..claim.position.1 + claim.dimensions.1 {
        fields[i][j] += 1;
      }
    }
  }

  let mut count = 0;
  for i in 0..field_size {
    for j in 0..field_size {
      if fields[i][j] > 1 {
        count += 1;
      }
    }
  }
  count
}

fn find_intact_claim(claims: &Vec<Claim>, field_size: usize) -> usize {
  let mut fields = init_fields(field_size, field_size);
  for claim in claims {
    for i in claim.position.0..claim.position.0 + claim.dimensions.0 {
      for j in claim.position.1..claim.position.1 + claim.dimensions.1 {
        fields[i][j] += 1;
      }
    }
  }

  for claim in claims {
    if is_intact(&fields, &claim) {
      return claim.id;
    }
  }

  0
}

fn is_intact(fields: &Vec<Vec<usize>>, claim: &Claim) -> bool {
  let mut is_intact = true;
  for i in claim.position.0..claim.position.0 + claim.dimensions.0 {
    for j in claim.position.1..claim.position.1 + claim.dimensions.1 {
      if !is_intact {
        return false;
      }
      is_intact = is_intact && fields[i][j] == 1;
    }
  }
  is_intact
}

struct Claim {
  id: usize,
  position: (usize, usize),
  dimensions: (usize, usize),
}
impl Claim {
  pub fn new(
    id: usize,
    position: (usize, usize),
    dimensions: (usize, usize),
  ) -> Claim {
    Claim {
      id,
      position,
      dimensions,
    }
  }
}

fn init_fields(w: usize, h: usize) -> Vec<Vec<usize>> {
  vec![vec![0; h]; w]
}

fn read_input(filename: &str) -> Vec<Claim> {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  let re = Regex::new(r"\#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");
  content
    .split_terminator('\n')
    .map(|s| {
      let caps = re.captures(&s).unwrap();
      Claim::new(
        caps[1].parse::<usize>().unwrap(),
        (
          caps[2].parse::<usize>().unwrap(),
          caps[3].parse::<usize>().unwrap(),
        ),
        (
          caps[4].parse::<usize>().unwrap(),
          caps[5].parse::<usize>().unwrap(),
        ),
      )
    })
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let claims = read_input("test-input.txt");
    assert_eq!(count_used_field_more_than_once(&claims, 8), 4);
  }

  #[test]
  fn part2_test() {
    let claims = read_input("test-input.txt");
    assert_eq!(find_intact_claim(&claims, 8), 3);
  }
}
