use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut device = read_input("input.txt");
  device.run_til_halt();
  println!(
    "Day 19: Go With The Flow part1 solution\n{:?}",
    device.reg[0]
  );

  // device.reset();
  // device.reg[0] = 1;
  // device.run_til_halt();
  // let part_2 = device.reg[0];

  let limit = 10551309;
  let mut factors = Vec::new();
  for i in 1..=limit as usize {
    if limit % i == 0 {
      factors.push(i);
      if limit / i == i {
        factors.push(limit / i);
      }
    }
  }
  let part_2 = factors.iter().fold(0, |acc, n| acc + n);
  println!("Day 19: Go With The Flow part2 solution\n{:?}", part_2);
}

#[derive(Debug)]
struct Device {
  ip: usize,
  ip_reg: usize,
  reg: Vec<usize>,
  instructions: Vec<Op>,
}
impl Device {
  pub fn new(ip_reg: usize, instructions: Vec<Op>) -> Self {
    Self {
      ip: 0,
      ip_reg,
      reg: vec![0; 6],
      instructions,
    }
  }

  pub fn run_til_halt(&mut self) {
    while self.ip < self.instructions.len() {
      self.execute_cmd();
    }
  }

  pub fn execute_cmd(&mut self) {
    self.reg[self.ip_reg] = self.ip;
    let command = &self.instructions[self.ip];
    let a = command.args[0];
    let b = command.args[1];
    let c = command.args[2];
    match command.op_code.as_str() {
      "addr" => self.reg[c] = self.reg[a] + self.reg[b],
      "addi" => self.reg[c] = self.reg[a] + b,
      "mulr" => self.reg[c] = self.reg[a] * self.reg[b],
      "muli" => self.reg[c] = self.reg[a] * b,
      "banr" => self.reg[c] = self.reg[a] & self.reg[b],
      "bani" => self.reg[c] = self.reg[a] & b,
      "borr" => self.reg[c] = self.reg[a] | self.reg[b],
      "bori" => self.reg[c] = self.reg[a] | b,
      "setr" => self.reg[c] = self.reg[a],
      "seti" => self.reg[c] = a,
      "gtir" => self.reg[c] = if a > self.reg[b] { 1 } else { 0 },
      "gtri" => self.reg[c] = if self.reg[a] > b { 1 } else { 0 },
      "gtrr" => self.reg[c] = if self.reg[a] > self.reg[b] { 1 } else { 0 },
      "eqir" => self.reg[c] = if a == self.reg[b] { 1 } else { 0 },
      "eqri" => self.reg[c] = if self.reg[a] == b { 1 } else { 0 },
      "eqrr" => self.reg[c] = if self.reg[a] == self.reg[b] { 1 } else { 0 },
      _ => panic!("not expected {}", command.op_code),
    }
    self.ip = self.reg[self.ip_reg];
    self.ip += 1;
  }

  pub fn reset(&mut self) {
    self.ip = 0;
    self.reg = vec![0; 6];
  }
}

#[derive(Debug)]
struct Op {
  op_code: String,
  args: Vec<usize>,
}
impl Op {
  pub fn new(op_code: String, args: Vec<usize>) -> Self {
    Self { op_code, args }
  }
}

fn read_input(filename: &str) -> Device {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");

  let rows = content.split_terminator('\n').collect::<Vec<&str>>();
  let ip_reg = rows[0].chars().last().unwrap() as usize - '0' as usize;
  let mut instructions = Vec::new();
  for r in rows.iter().skip(1) {
    let items = r.split_terminator(' ').collect::<Vec<&str>>();
    instructions.push(Op::new(
      items[0].to_string(),
      vec![
        items[1].parse::<usize>().unwrap(),
        items[2].parse::<usize>().unwrap(),
        items[3].parse::<usize>().unwrap(),
      ],
    ))
  }
  Device::new(ip_reg, instructions)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let mut device = read_input("test-input.txt");
    device.run_til_halt();
    assert_eq!(device.reg[0], 6);
  }
}
