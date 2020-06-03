use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut fields = read_file("input.txt");

  let (total, retained) = fields.fill_clay();
  print_section(&fields);
  println!("Day 17: Reservoir Research part1 solution\n{:?}", total);

  println!("Day 17: Reservoir Research part2 solution\n{:?}", retained);
}

struct Fields {
  flow_positions: Vec<(i32, i32)>,
  min_x: i32,
  max_x: i32,
  min_y: i32,
  max_y: i32,
  map: HashMap<(i32, i32), char>,
}
impl Fields {
  pub fn new() -> Self {
    let mut map = HashMap::new();
    let water_spring_pos = (500, 0);
    map.insert(water_spring_pos.clone(), '+');
    Self {
      flow_positions: vec![water_spring_pos],
      min_x: i32::max_value(),
      max_x: 0,
      min_y: i32::max_value(),
      max_y: 0,
      map,
    }
  }

  pub fn fill_clay(&mut self) -> (i32, i32) {
    let mut total_water_count = 0;
    let mut retained_water_count = 0;
    while !self.flow_positions.is_empty() {
      for _ in 0..self.flow_positions.len() {
        let (x, y) = self.flow_positions.remove(0);
        let next_y = y + 1;
        let ch = self.map.get(&(x, next_y)).or(Some(&'.')).unwrap();
        if *ch == '.' {
          self.map.insert((x, next_y), '|');
          if next_y >= self.min_y {
            total_water_count += 1;
          }
          if next_y < self.max_y {
            self.flow_positions.push((x, next_y));
          }
        } else if *ch == '#' || *ch == '~' {
          let left_edge = self.find_edge((x, y), Direction::Left);
          let right_edge = self.find_edge((x, y), Direction::Right);

          // fill row
          let mut positions = Vec::new();
          let mut water_char = '|';
          if left_edge.1 == right_edge.1 {
            if left_edge.1 == '#' {
              water_char = '~';
              positions.push((x, y - 1));
            } else {
              positions.push((left_edge.0, y));
              positions.push((right_edge.0, y));
            }
          } else {
            let next_x = if left_edge.1 == '#' {
              right_edge.0
            } else {
              left_edge.0
            };
            positions.push((next_x, y));
          };

          let correction_left = if left_edge.1 == '#' { 1 } else { 0 };
          let from = left_edge.0 + correction_left;
          let correction_right = if right_edge.1 == '#' { -1 } else { 0 };
          let to = right_edge.0 + correction_right;

          for i in from..=to {
            let curr = self.map.get(&(i, y)).or(Some(&'.')).unwrap();
            if *curr != '~' && *curr != '|' {
              total_water_count += 1;
            }
            if *curr != '~' && water_char == '~' {
              retained_water_count += 1;
            }
            self.map.insert((i, y), water_char);
          }
          self.flow_positions.append(&mut positions);
        }
      }
    }
    (total_water_count, retained_water_count)
  }

  pub fn find_edge(&mut self, pos: (i32, i32), dir: Direction) -> (i32, char) {
    let (x, y) = pos;
    let mut curr_x = x;
    let mut edge = (0, ' ');
    loop {
      curr_x += if dir == Direction::Left { -1 } else { 1 };
      let curr_char = self.map.get(&(curr_x, y)).or(Some(&'.')).unwrap();
      let bellow_char = self.map.get(&(curr_x, y + 1)).or(Some(&'.')).unwrap();
      if *curr_char == '#' {
        edge = (curr_x, *curr_char);
      } else if *bellow_char == '.' {
        edge = (curr_x, *bellow_char);
      }
      if edge.1 != ' ' {
        return edge;
      }
    }
  }
}

#[derive(PartialEq)]
enum Direction {
  Left,
  Right,
}

fn read_file(filename: &str) -> Fields {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");

  let re_x = Regex::new(r"x=(\d+),\sy=(\d+)..(\d+)").unwrap();
  let re_y = Regex::new(r"y=(\d+),\sx=(\d+)..(\d+)").unwrap();
  let mut fields = Fields::new();
  for s in content.split_terminator('\n') {
    let mut captures = re_x.captures(s);
    if let Some(caps) = captures {
      let x = caps[1].parse::<i32>().unwrap();
      let y_from = caps[2].parse::<i32>().unwrap();
      let y_to = caps[3].parse::<i32>().unwrap();
      if x < fields.min_x {
        fields.min_x = x;
      }
      if x > fields.max_x {
        fields.max_x = x;
      }
      if y_from < fields.min_y {
        fields.min_y = y_from;
      }
      if y_to > fields.max_y {
        fields.max_y = y_to;
      }
      for y in y_from..=y_to {
        fields.map.insert((x, y), '#');
      }
    } else {
      captures = re_y.captures(s);
      let caps = captures.unwrap();
      let y = caps[1].parse::<i32>().unwrap();
      let x_from = caps[2].parse::<i32>().unwrap();
      let x_to = caps[3].parse::<i32>().unwrap();
      if y < fields.min_y {
        fields.min_y = y;
      }
      if y > fields.max_y {
        fields.max_y = y;
      }
      if x_from < fields.min_x {
        fields.min_x = x_from;
      }
      if x_to > fields.max_x {
        fields.max_x = x_to;
      }
      for x in x_from..=x_to {
        fields.map.insert((x, y), '#');
      }
    }
  }

  fields
}

fn print_section(fields: &Fields) {
  let mut s = String::new();
  for y in fields.min_y - 1..=fields.max_y + 1 {
    for x in fields.min_x - 1..=fields.max_x + 1 {
      if let Some(ch) = fields.map.get(&(x, y)) {
        s.push(*ch);
      } else {
        s.push('.');
      }
    }
    s.push('\n');
  }
  println!("{}", s);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let mut fields = read_file("test-input.txt");
    let (total, retained) = fields.fill_clay();
    print_section(&fields);
    assert_eq!(total, 57);
    assert_eq!(retained, 29);
  }
}
