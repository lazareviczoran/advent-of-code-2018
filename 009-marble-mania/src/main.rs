use std::collections::VecDeque;

fn main() {
  // Input:
  // 459 players; last marble is worth 72103 points

  println!(
    "Day 9: Marble Mania part1 solution\n {}",
    find_high_score(72103, 459)
  );

  println!(
    "Day 9: Marble Mania part2 solution\n {}",
    find_high_score(7210300, 459)
  );
}

fn find_high_score(last_marble: usize, players: usize) -> usize {
  let mut circle = VecDeque::new();
  circle.push_front(0);
  let mut player_points = vec![0; players];
  for i in 1..=last_marble {
    if i % 23 == 0 {
      circle.rotate_right(7);
      let val = circle.pop_front().expect("Couldn't pop value");
      player_points[i % players] += i + val;
    } else {
      (0..2).for_each(|_| {
        let item = circle.pop_front().expect("Couldn't pop value");
        circle.push_back(item);
      });
      circle.push_front(i);
    }
  }

  *player_points.iter().max().expect("Couldn't find max value")
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test1() {
    assert_eq!(find_high_score(25, 9), 32);
  }

  #[test]
  fn part1_test2() {
    assert_eq!(find_high_score(1618, 10), 8317);
  }

  #[test]
  fn part1_test3() {
    assert_eq!(find_high_score(7999, 13), 146373);
  }

  #[test]
  fn part1_test4() {
    assert_eq!(find_high_score(1104, 17), 2764);
  }

  #[test]
  fn part1_test5() {
    assert_eq!(find_high_score(6111, 21), 54718);
  }

  #[test]
  fn part1_test6() {
    assert_eq!(find_high_score(5807, 30), 37305);
  }
}
