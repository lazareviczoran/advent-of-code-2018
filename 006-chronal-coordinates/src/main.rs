use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let (positions, _, bottom_right_pos) = read_input("input.txt");

  println!(
    "Day 6: Chronal Coordinates part1 solution\n {}",
    find_largest_finite_area(&positions, bottom_right_pos)
  );

  println!(
    "Day 6: Chronal Coordinates part2 solution\n {}",
    find_area_of_region_containing_all(&positions, bottom_right_pos, 10000)
  );
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
  x: i32,
  y: i32,
  id: i32,
}
impl Position {
  pub fn new(id: i32, x: i32, y: i32) -> Position {
    Position { id, x, y }
  }
}

fn find_area_of_region_containing_all(
  positions: &Vec<Position>,
  bottom_right_pos: Position,
  total_dist_limit: i32,
) -> i32 {
  let additional_fields = 100;
  let width = bottom_right_pos.x as usize + additional_fields;
  let height = bottom_right_pos.y as usize + additional_fields;
  let mut fields = vec![vec![0; height + 1]; width + 1];
  let mut region_area = 0;
  for y in 0..=height {
    for x in 0..=width {
      let curr_pos = Position::new(0, x as i32, y as i32);
      if is_distance_to_all_bellow_limit(positions, curr_pos, total_dist_limit)
      {
        fields[x][y] = 1;
        region_area += 1;
      }
    }
  }

  // print_fields(&fields);

  region_area
}

fn is_distance_to_all_bellow_limit(
  positions: &Vec<Position>,
  curr_pos: Position,
  total_dist_limit: i32,
) -> bool {
  let mut total_dist = 0;
  for p in positions {
    if total_dist >= total_dist_limit {
      return false;
    }
    let distance = (curr_pos.x - p.x).abs() + (curr_pos.y - p.y).abs();
    total_dist += distance;
  }
  total_dist < total_dist_limit
}

fn find_largest_finite_area(
  positions: &Vec<Position>,
  bottom_right_pos: Position,
) -> i32 {
  let additional_fields = 100;
  let width = bottom_right_pos.x as usize + additional_fields;
  let height = bottom_right_pos.y as usize + additional_fields;
  let mut fields = vec![vec![0; height + 1]; width + 1];
  let mut infinite_area_points = HashSet::new();
  let mut point_areas = HashMap::new();
  for y in 0..=height {
    for x in 0..=width {
      let curr_pos = Position::new(0, x as i32, y as i32);
      let closest_points = find_closest_points(positions, curr_pos);
      if closest_points.len() == 1 {
        // if there is only one closest point, set its order number as field
        // value, otherwise leave it with value 0
        fields[x][y] = closest_points[0].id;
      }
      if x == 0 || x == width || y == 0 || y == height {
        infinite_area_points.insert(fields[x][y]);
      }
      if let Some(area) = point_areas.get_mut(&fields[x][y]) {
        *area += 1;
      } else {
        point_areas.insert(fields[x][y], 1);
      }
    }
  }

  // print_fields(&fields);

  // find largest area among finite areas
  let mut largest_area = 0;
  for (id, area) in point_areas.iter() {
    if !infinite_area_points.contains(id) && largest_area < *area {
      largest_area = *area;
    }
  }

  largest_area
}

fn find_closest_points(
  positions: &Vec<Position>,
  curr_pos: Position,
) -> Vec<Position> {
  let mut closest_points = vec![];
  let mut smallest_dist = i32::max_value();

  for p in positions {
    let distance = (curr_pos.x - p.x).abs() + (curr_pos.y - p.y).abs();
    if distance < smallest_dist {
      smallest_dist = distance;
      closest_points = vec![*p];
    } else if distance == smallest_dist {
      closest_points.push(*p);
    }
  }
  closest_points
}

fn print_fields(fields: &Vec<Vec<i32>>) {
  let mut output = String::new();
  for y in 0..fields[0].len() {
    for x in 0..fields.len() {
      output.push((64 + &fields[x][y]) as u8 as char);
    }
    output.push('\n');
  }
  println!("{}", output);
}

fn read_input(filename: &str) -> (Vec<Position>, Position, Position) {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");

  let mut upper_left_pos: Option<Position> = None;
  let mut bottom_right_pos: Option<Position> = None;
  let mut i = 1;
  let positions = content
    .split_terminator('\n')
    .map(|s| {
      let positions: Vec<i32> = s
        .split_terminator(", ")
        .map(|p| p.parse::<i32>().unwrap())
        .collect();
      let new_pos = Position::new(i, positions[0], positions[1]);
      if let Some(upper_left) = &upper_left_pos {
        if new_pos.x <= upper_left.x && new_pos.y <= upper_left.y {
          upper_left_pos = Some(new_pos);
        }
      } else {
        upper_left_pos = Some(new_pos);
      }
      if let Some(bottom_right) = &bottom_right_pos {
        if new_pos.x >= bottom_right.x && new_pos.y >= bottom_right.y {
          bottom_right_pos = Some(new_pos);
        }
      } else {
        bottom_right_pos = Some(new_pos);
      }
      i += 1;
      new_pos
    })
    .collect();
  (
    positions,
    upper_left_pos.unwrap(),
    bottom_right_pos.unwrap(),
  )
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let (positions, _, bottom_right_pos) = read_input("test-input.txt");
    assert_eq!(find_largest_finite_area(&positions, bottom_right_pos), 17);
  }

  #[test]
  fn part2_test() {
    let (positions, _, bottom_right_pos) = read_input("test-input.txt");
    assert_eq!(
      find_area_of_region_containing_all(&positions, bottom_right_pos, 32),
      16
    );
  }
}
