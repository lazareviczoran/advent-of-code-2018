#![feature(drain_filter)]
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let (map, carts) = read_input("input.txt");

  println!(
    "Day 13: Mine Cart Madness part1 solution\n{:?}",
    find_first_collision(&map, &carts)
  );

  println!(
    "Day 13: Mine Cart Madness part2 solution\n{:?}",
    find_last_remaining_cart_pos(&map, &carts)
  );
}

fn find_last_remaining_cart_pos(
  map: &Vec<Vec<char>>,
  initial_carts: &Vec<Cart>,
) -> (usize, usize) {
  let mut carts = initial_carts.clone();
  carts.sort();

  while carts.len() > 1 {
    apply_tick_v2(map, &mut carts);
  }
  carts[0].pos
}

fn apply_tick_v2(map: &Vec<Vec<char>>, carts: &mut Vec<Cart>) {
  let clone = carts.clone();
  carts.clear();
  let mut visited = HashSet::new();
  for cart in clone.iter() {
    visited.insert(cart.pos);
  }
  for cart in clone.iter() {
    let mut cart_mut = *cart;
    cart_mut.move_pos(map);
    if visited.get(&cart_mut.pos).is_none() {
      carts.push(cart_mut);
      visited.insert(cart_mut.pos);
    } else {
      carts.drain_filter(|c| c.pos == cart_mut.pos);
    }
  }
  carts.sort();
}

fn find_first_collision(
  map: &Vec<Vec<char>>,
  initial_carts: &Vec<Cart>,
) -> (usize, usize) {
  let mut carts = initial_carts.clone();
  carts.sort();

  loop {
    let collisions = apply_tick(map, &mut carts);
    if collisions.len() > 0 {
      return collisions[0];
    }
  }
}

fn apply_tick(
  map: &Vec<Vec<char>>,
  carts: &mut Vec<Cart>,
) -> Vec<(usize, usize)> {
  for cart in carts.iter_mut() {
    cart.move_pos(map);
  }
  carts.sort();
  let mut collisions = Vec::new();
  for i in 0..carts.len() - 1 {
    if carts[i].pos == carts[i + 1].pos {
      collisions.push(carts[i].pos);
    }
  }
  collisions
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Directions {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cart {
  pos: (usize, usize),
  dir: Directions,
  intersection_count: usize,
}
impl Cart {
  pub fn new(pos: (usize, usize), dir: Directions) -> Self {
    Self {
      pos,
      dir,
      intersection_count: 0,
    }
  }

  pub fn move_pos(&mut self, map: &Vec<Vec<char>>) {
    let (x, y) = self.pos;
    let mut next_x = x;
    let mut next_y = y;
    match self.dir {
      Directions::UP => next_y -= 1,
      Directions::DOWN => next_y += 1,
      Directions::LEFT => next_x -= 1,
      Directions::RIGHT => next_x += 1,
    }
    self.pos = (next_x, next_y);
    let next_ch = map[next_x][next_y];
    match next_ch {
      '\\' => match self.dir {
        Directions::DOWN => self.dir = Directions::RIGHT,
        Directions::UP => self.dir = Directions::LEFT,
        Directions::LEFT => self.dir = Directions::UP,
        Directions::RIGHT => self.dir = Directions::DOWN,
      },
      '/' => match self.dir {
        Directions::DOWN => self.dir = Directions::LEFT,
        Directions::UP => self.dir = Directions::RIGHT,
        Directions::LEFT => self.dir = Directions::DOWN,
        Directions::RIGHT => self.dir = Directions::UP,
      },
      '+' => {
        match self.intersection_count % 3 {
          0 => match self.dir {
            Directions::DOWN => self.dir = Directions::RIGHT,
            Directions::UP => self.dir = Directions::LEFT,
            Directions::LEFT => self.dir = Directions::DOWN,
            Directions::RIGHT => self.dir = Directions::UP,
          },
          2 => match self.dir {
            Directions::DOWN => self.dir = Directions::LEFT,
            Directions::UP => self.dir = Directions::RIGHT,
            Directions::LEFT => self.dir = Directions::UP,
            Directions::RIGHT => self.dir = Directions::DOWN,
          },
          _ => {
            // keep going straight, no changes required
          }
        };
        self.intersection_count += 1;
      }
      _ => {
        // keep going straight, no changes required
      }
    }
  }
}
impl PartialOrd for Cart {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl Ord for Cart {
  fn cmp(&self, other: &Self) -> Ordering {
    let (x, y) = self.pos;
    let (other_x, other_y) = other.pos;
    y.cmp(&other_y).then(x.cmp(&other_x))
  }
}

fn read_input(filename: &str) -> (Vec<Vec<char>>, Vec<Cart>) {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");

  let mut carts = Vec::new();
  let map = content
    .split_terminator('\n')
    .fold(Vec::new(), |mut acc, s| {
      let mut chars = s.chars();
      let mut i = 0;
      while let Some(ch) = chars.next() {
        if acc.len() == i {
          acc.push(Vec::new());
        }
        match ch {
          '<' | '>' | 'v' | '^' => {
            let dir = match ch {
              '<' => Directions::LEFT,
              '>' => Directions::RIGHT,
              'v' => Directions::DOWN,
              '^' => Directions::UP,
              _ => panic!("Something really went wrong {}", ch),
            };
            let map_value;
            if dir == Directions::LEFT || dir == Directions::RIGHT {
              map_value = '-';
            } else {
              map_value = '|';
            }

            carts.push(Cart::new((i, acc[i].len()), dir));
            acc[i].push(map_value);
          }
          _ => acc[i].push(ch),
        };
        i += 1;
      }
      acc
    });

  (map, carts)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let (map, carts) = read_input("test-input.txt");

    assert_eq!(find_first_collision(&map, &carts), (7, 3));
  }

  #[test]
  fn part2_test() {
    let (map, carts) = read_input("test-input2.txt");

    assert_eq!(find_last_remaining_cart_pos(&map, &carts), (6, 4));
  }
}
