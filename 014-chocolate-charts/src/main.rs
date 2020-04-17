fn main() {
  let input = 637061;

  println!(
    "Day 14: Chocolate Charts part1 solution\n{:?}",
    find_score_after_nth_recipe(input)
  );

  println!(
    "Day 14: Chocolate Charts part2 solution\n{:?}",
    find_num_of_recipes_before(input)
  );
}

fn find_score_after_nth_recipe(n: usize) -> usize {
  let mut recipes = vec![3, 7];
  let mut elf1 = 0;
  let mut elf2 = 1;
  while recipes.len() < n + 10 {
    let sum = recipes[elf1] + recipes[elf2];
    if sum < 10 {
      recipes.push(sum);
    } else {
      recipes.push(sum / 10);
      recipes.push(sum % 10);
    }
    elf1 = (elf1 + 1 + recipes[elf1]) % recipes.len();
    elf2 = (elf2 + 1 + recipes[elf2]) % recipes.len();
  }
  get_score(&recipes, n, 10)
}

fn get_score(recipes: &Vec<usize>, start: usize, range: usize) -> usize {
  let mut res = 0;
  for i in 0..range {
    res *= 10;
    res += recipes[start + i];
  }
  res
}

fn find_num_of_recipes_before(n: usize) -> usize {
  let mut recipes = vec![3, 7];
  let mut elf1 = 0;
  let mut elf2 = 1;
  let n_len = n.to_string().len();
  let mut current_seq = 0;
  while current_seq != n {
    let sum = recipes[elf1] + recipes[elf2];
    if sum < 10 {
      recipes.push(sum);
    } else {
      recipes.push(sum / 10);
      if recipes.len() >= n_len {
        current_seq = get_score(&recipes, recipes.len() - n_len, n_len);
        if current_seq == n {
          break;
        }
      }
      recipes.push(sum % 10);
    }
    let length = recipes.len();
    elf1 = (elf1 + 1 + recipes[elf1]) % length;
    elf2 = (elf2 + 1 + recipes[elf2]) % length;
    if length >= n_len {
      current_seq = get_score(&recipes, length - n_len, n_len);
    }
  }
  let length = recipes.len();
  length - n_len
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test1() {
    assert_eq!(find_score_after_nth_recipe(9), 5158916779);
  }

  #[test]
  fn part1_test2() {
    assert_eq!(find_score_after_nth_recipe(5), 0124515891);
  }

  #[test]
  fn part1_test3() {
    assert_eq!(find_score_after_nth_recipe(18), 9251071085);
  }

  #[test]
  fn part1_test4() {
    assert_eq!(find_score_after_nth_recipe(2018), 5941429882);
  }

  #[test]
  fn part2_test1() {
    assert_eq!(find_num_of_recipes_before(9), 13);
  }

  #[test]
  fn part2_test2() {
    assert_eq!(find_num_of_recipes_before(5), 9);
  }

  #[test]
  fn part2_test3() {
    assert_eq!(find_num_of_recipes_before(18), 48);
  }

  #[test]
  fn part2_test4() {
    assert_eq!(find_num_of_recipes_before(2018), 86764);
  }
}
