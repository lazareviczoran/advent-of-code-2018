use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let (mut map, start_pos) = read_input("input.txt");
  let distances = find_max_door_pass(&mut map, &start_pos);
  println!(
    "Day 20: A Regular Map part1 solution\n{:?}",
    distances.last().unwrap()
  );

  println!(
    "Day 20: A Regular Map part2 solution\n{:?}",
    distances
      .iter()
      .fold(0, |acc, d| if *d >= 1000 { acc + 1 } else { acc })
  );
}

fn find_max_door_pass(
  map: &mut Vec<Vec<char>>,
  start_pos: &(usize, usize),
) -> Vec<usize> {
  let (x, y) = start_pos;
  let mut queue = vec![(*x, *y, 0)];
  let mut visited = vec![vec![false; map[0].len()]; map.len()];
  let mut distances = Vec::new();
  while !queue.is_empty() {
    let (curr_x, curr_y, curr_dist) = queue.remove(0);
    map[curr_x][curr_y] = 'O';
    visited[curr_x][curr_y] = true;
    distances.push(curr_dist);

    if curr_x + 2 < map.len()
      && map[curr_x + 1][curr_y] == '|'
      && !visited[curr_x + 2][curr_y]
    {
      queue.push((curr_x + 2, curr_y, curr_dist + 1));
    }
    if curr_x >= 2
      && map[curr_x - 1][curr_y] == '|'
      && !visited[curr_x - 2][curr_y]
    {
      queue.push((curr_x - 2, curr_y, curr_dist + 1));
    }
    if curr_y + 2 < map.len()
      && map[curr_x][curr_y + 1] == '-'
      && !visited[curr_x][curr_y + 2]
    {
      queue.push((curr_x, curr_y + 2, curr_dist + 1));
    }
    if curr_y >= 2
      && map[curr_x][curr_y - 1] == '-'
      && !visited[curr_x][curr_y - 2]
    {
      queue.push((curr_x, curr_y - 2, curr_dist + 1));
    }
  }
  distances
}

fn print_map(map: &Vec<Vec<char>>) {
  let mut s = String::new();
  for y in 0..map[0].len() {
    for x in 0..map.len() {
      s.push(map[x][y]);
    }
    s.push('\n');
  }
  println!("{}", s);
}

fn read_input(filename: &str) -> (Vec<Vec<char>>, (usize, usize)) {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");

  let mut chars = content.chars().peekable();
  chars.next();
  let mut directions = HashMap::new();
  directions.insert('N', (0, -1, '-'));
  directions.insert('S', (0, 1, '-'));
  directions.insert('E', (1, 0, '|'));
  directions.insert('W', (-1, 0, '|'));

  let mut map = HashMap::new();
  let mut min_x = 0i32;
  let mut max_x = 0i32;
  let mut min_y = 0i32;
  let mut max_y = 0i32;
  let mut curr_x = 0i32;
  let mut curr_y = 0i32;
  let mut stack = Vec::new();
  map.entry((curr_x, curr_y)).or_insert('X');
  while let Some(curr) = chars.next() {
    match curr {
      'N' | 'S' | 'E' | 'W' => {
        let (diff_x, diff_y, door_char) = directions.get(&curr).unwrap();
        curr_x += diff_x;
        curr_y += diff_y;
        map.entry((curr_x, curr_y)).or_insert(*door_char);
        curr_x += diff_x;
        curr_y += diff_y;
        map.entry((curr_x, curr_y)).or_insert('.');
        if curr_y < min_y {
          min_y = curr_y;
        } else if curr_y > max_y {
          max_y = curr_y;
        }
        if curr_x < min_x {
          min_x = curr_x;
        } else if curr_x > max_x {
          max_x = curr_x;
        }
      }
      '(' => {
        stack.push((curr_x, curr_y));
      }
      '|' => {
        let (x, y) = stack[stack.len() - 1];
        curr_x = x;
        curr_y = y
      }
      ')' => {
        stack.pop();
      }
      '^' | '$' => {
        // skip
      }
      _ => panic!("Unexpected char {}", curr),
    }
  }

  let mut res = Vec::new();
  let mut curr_x = 0;
  for x in min_x - 1..=max_x + 1 {
    res.push(Vec::new());
    for y in min_y - 1..=max_y + 1 {
      let ch = map.get(&(x, y)).unwrap_or(&'#');
      res[curr_x].push(*ch);
    }
    curr_x += 1;
  }

  (res, (min_x.abs() as usize + 1, min_y.abs() as usize + 1))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_test1() {
    let (mut map, start_pos) = read_input("test-input.txt");
    assert_eq!(*find_max_door_pass(&mut map, &start_pos).last().unwrap(), 3);
  }

  #[test]
  fn part1_test2() {
    let (mut map, start_pos) = read_input("test-input2.txt");
    assert_eq!(
      *find_max_door_pass(&mut map, &start_pos).last().unwrap(),
      10
    );
  }

  #[test]
  fn part1_test3() {
    let (mut map, start_pos) = read_input("test-input3.txt");
    assert_eq!(
      *find_max_door_pass(&mut map, &start_pos).last().unwrap(),
      18
    );
  }

  #[test]
  fn part1_test4() {
    let (mut map, start_pos) = read_input("test-input4.txt");
    assert_eq!(
      *find_max_door_pass(&mut map, &start_pos).last().unwrap(),
      23
    );
  }

  #[test]
  fn part1_test5() {
    let (mut map, start_pos) = read_input("test-input5.txt");
    assert_eq!(
      *find_max_door_pass(&mut map, &start_pos).last().unwrap(),
      31
    );
  }
}
