fn main() {
  // input: 9798
  let grid = generate_grid(9798);

  println!(
    "Day 11: Chronal Charge part1 solution\n{:?}",
    find_best_fuel_square_3x3(&grid)
  );

  println!(
    "Day 11: Chronal Charge part2 solution\n{:?}",
    find_best_fuel_square(&grid)
  );
}

fn find_best_fuel_square(grid: &Vec<Vec<i32>>) -> (usize, usize, usize) {
  let size = grid.len();
  let mut sums = grid.clone();
  let mut best = (1, 1, 1);
  let mut best_sum = i32::min_value();
  for y in 1..size {
    for x in 1..size {
      sums[x][y] =
        grid[x][y] + sums[x][y - 1] + sums[x - 1][y] - sums[x - 1][y - 1];
    }
  }
  for cell_size in 1..size {
    for y in 1..size - cell_size {
      for x in 1..size - cell_size {
        let prev_x = x - 1;
        let prev_y = y - 1;
        let sum = sums[prev_x][prev_y]
          + sums[prev_x + cell_size][prev_y + cell_size]
          - sums[prev_x + cell_size][prev_y]
          - sums[prev_x][prev_y + cell_size];
        if sum > best_sum {
          best_sum = sum;
          best = (x, y, cell_size);
        }
      }
    }
  }

  best
}

fn find_best_fuel_square_3x3(grid: &Vec<Vec<i32>>) -> (usize, usize) {
  let mut best = (1, 1);
  let mut best_sum = i32::min_value();
  let size = grid.len() - 1;
  for y in 1..=size - 3 {
    for x in 1..=size - 3 {
      let mut sum = 0;
      for j in 0..3 {
        for i in 0..3 {
          sum += grid[x + i][y + j];
        }
      }
      if sum > best_sum {
        best_sum = sum;
        best = (x, y);
      }
    }
  }

  best
}

fn generate_grid(grid_sn: usize) -> Vec<Vec<i32>> {
  let size = 300;
  let mut grid = vec![vec![0; size + 1]; size + 1];
  for y in 1..=size {
    for x in 1..=size {
      let power_level = calculate_fuel_cell_power(x, y, grid_sn);
      grid[x][y] = power_level;
    }
  }

  grid
}

fn calculate_fuel_cell_power(x: usize, y: usize, grid_sn: usize) -> i32 {
  let rack_id = x + 10;
  let power_level = (((rack_id * y + grid_sn) * rack_id) / 100 % 10) as i32 - 5;
  power_level
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn calculate_fuel_power() {
    assert_eq!(calculate_fuel_cell_power(3, 5, 8), 4);
    assert_eq!(calculate_fuel_cell_power(122, 79, 57), -5);
    assert_eq!(calculate_fuel_cell_power(217, 196, 39), 0);
    assert_eq!(calculate_fuel_cell_power(101, 153, 71), 4);
  }

  #[test]
  fn part1_case1() {
    assert_eq!(find_best_fuel_square_3x3(&generate_grid(18)), (33, 45));
  }

  #[test]
  fn part1_case2() {
    assert_eq!(find_best_fuel_square_3x3(&generate_grid(42)), (21, 61));
  }

  #[test]
  fn part2_case1() {
    assert_eq!(find_best_fuel_square(&generate_grid(18)), (90, 269, 16));
  }

  #[test]
  fn part2_case2() {
    assert_eq!(find_best_fuel_square(&generate_grid(42)), (232, 251, 12));
  }
}
