use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  // let points = read_input("test-input.txt");
  let points = read_input("input.txt");

  let (message, time_spent) = find_message(&points);
  println!("Day 10: The Stars Align part1 solution\n{}", message);

  println!("Day 10: The Stars Align part2 solution\n {}", time_spent);
}

fn find_message(init_points: &Vec<Point>) -> (String, usize) {
  let mut points = init_points.clone();
  let mut res = init_points.clone();
  let (min_x, max_x, min_y, max_y) = find_edge_points(&points);
  let mut min_diff_x = max_x - min_x;
  let mut min_diff_y = max_y - min_y;
  let mut time = 0;
  loop {
    move_points(&mut points);
    let (min_x, max_x, min_y, max_y) = find_edge_points(&points);
    let diff_x = max_x - min_x;
    let diff_y = max_y - min_y;
    if diff_x > min_diff_x && diff_y > min_diff_y {
      break;
    }
    if diff_x < min_diff_x {
      min_diff_x = diff_x;
    }
    if diff_y < min_diff_y {
      min_diff_y = diff_y;
    }
    res = points.clone();
    time += 1;
  }

  (prepare_message(&res), time)
}

fn move_points(points: &mut Vec<Point>) {
  for i in 0..points.len() {
    points[i].pos_x += points[i].vel_x;
    points[i].pos_y += points[i].vel_y;
  }
}

fn prepare_message(points: &Vec<Point>) -> String {
  let (min_x, max_x, min_y, max_y) = find_edge_points(&points);
  let mut fields = vec![
    vec![' '; (max_y - min_y).abs() as usize + 1];
    (max_x - min_x).abs() as usize + 1
  ];

  for p in points {
    fields[(p.pos_x - min_x) as usize][(p.pos_y - min_y) as usize] = '#';
  }

  let mut res = String::new();
  for y in 0..fields[0].len() {
    for x in 0..fields.len() {
      res.push(fields[x][y]);
    }
    res.push('\n');
  }
  res
}

fn find_edge_points(points: &Vec<Point>) -> (i32, i32, i32, i32) {
  let mut min_x = points[0].pos_x;
  let mut max_x = points[0].pos_x;
  let mut min_y = points[0].pos_y;
  let mut max_y = points[0].pos_y;
  for i in 1..points.len() {
    if points[i].pos_x < min_x {
      min_x = points[i].pos_x;
    }
    if points[i].pos_x > max_x {
      max_x = points[i].pos_x;
    }
    if points[i].pos_y < min_y {
      min_y = points[i].pos_y;
    }
    if points[i].pos_y > max_y {
      max_y = points[i].pos_y;
    }
  }
  (min_x, max_x, min_y, max_y)
}

#[derive(Debug, Clone, Copy)]
struct Point {
  pos_x: i32,
  pos_y: i32,
  vel_x: i32,
  vel_y: i32,
}
impl Point {
  pub fn new(pos_x: i32, pos_y: i32, vel_x: i32, vel_y: i32) -> Point {
    Point {
      pos_x,
      pos_y,
      vel_x,
      vel_y,
    }
  }
}

fn read_input(filename: &str) -> Vec<Point> {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");
  let re = Regex::new(
    r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>",
  )
  .unwrap();

  content
    .split_terminator('\n')
    .map(|s| {
      let caps = re.captures(s).unwrap();
      Point::new(
        caps[1].parse::<i32>().unwrap(),
        caps[2].parse::<i32>().unwrap(),
        caps[3].parse::<i32>().unwrap(),
        caps[4].parse::<i32>().unwrap(),
      )
    })
    .collect()
}
