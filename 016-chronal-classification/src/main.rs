#![feature(drain_filter)]
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let ops = read_file("input.txt");

  println!(
    "Day 16: Chronal Classification part1 solution\n{:?}",
    find_operations_count(&ops)
  );

  let operations_map = detect_operations(&ops);
  let test_commands = read_file("input_pt2.txt");
  println!(
    "Day 16: Chronal Classification part2 solution\n{:?}",
    run_test_program(&operations_map, &test_commands)
  );
}

fn run_test_program(
  op_map: &HashMap<usize, String>,
  commands: &Vec<Op>,
) -> usize {
  let mut registers = vec![0, 0, 0, 0];
  for c in commands {
    let command_code = op_map.get(&c.op_code).unwrap();
    execute_cmd(&command_code, &c, &mut registers);
  }
  registers[0]
}

fn execute_cmd(
  command_name: &String,
  command: &Op,
  registers: &mut Vec<usize>,
) {
  let a = command.args[0];
  let b = command.args[1];
  let c = command.args[2];
  match command_name.as_str() {
    "addr" => registers[c] = registers[a] + registers[b],
    "addi" => registers[c] = registers[a] + b,
    "mulr" => registers[c] = registers[a] * registers[b],
    "muli" => registers[c] = registers[a] * b,
    "banr" => registers[c] = registers[a] & registers[b],
    "bani" => registers[c] = registers[a] & b,
    "borr" => registers[c] = registers[a] | registers[b],
    "bori" => registers[c] = registers[a] | b,
    "setr" => registers[c] = registers[a],
    "seti" => registers[c] = a,
    "gtir" => registers[c] = if a > registers[b] { 1 } else { 0 },
    "gtri" => registers[c] = if registers[a] > b { 1 } else { 0 },
    "gtrr" => registers[c] = if registers[a] > registers[b] { 1 } else { 0 },
    "eqir" => registers[c] = if a == registers[b] { 1 } else { 0 },
    "eqri" => registers[c] = if registers[a] == b { 1 } else { 0 },
    "eqrr" => registers[c] = if registers[a] == registers[b] { 1 } else { 0 },
    _ => panic!("not expected {}", command_name),
  }
}

fn detect_operations(operations: &Vec<Op>) -> HashMap<usize, String> {
  let mut operations_map = HashMap::new();

  while operations_map.len() < 16 {
    for op in operations {
      let determined_ops =
        operations_map.values().cloned().collect::<Vec<String>>();
      let op_code = op.op_code;
      let mut op_candidates = find_potential_operations(&op);
      op_candidates.drain_filter(|a| determined_ops.contains(a));
      if op_candidates.len() == 1 {
        operations_map.insert(op_code, op_candidates[0].clone());
      }
    }
  }

  operations_map
}

fn find_operations_count(operations: &Vec<Op>) -> usize {
  let mut count = 0;
  for op in operations {
    if find_potential_operations(&op).len() > 2 {
      count += 1;
    }
  }
  count
}

fn find_potential_operations(operation: &Op) -> Vec<String> {
  let before_state = &operation.register_states[0];
  let after_state = &operation.register_states[1];
  let store_arg = operation.args[2];

  vec![
    "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr",
    "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr",
  ]
  .iter()
  .fold(Vec::new(), |mut acc, cmd| {
    let mut registers = before_state.clone();
    let cmd_name = String::from(*cmd);
    execute_cmd(&cmd_name, operation, &mut registers);
    if registers[store_arg] == after_state[store_arg] {
      acc.push(cmd_name);
    }
    acc
  })
}

#[derive(Debug)]
struct Op {
  op_code: usize,
  args: Vec<usize>,
  register_states: Vec<Vec<usize>>,
}
impl Op {
  pub fn new(op_code: usize, args: Vec<usize>) -> Self {
    Self {
      op_code,
      args,
      register_states: Vec::new(),
    }
  }
}

fn read_file(filename: &str) -> Vec<Op> {
  let mut file = File::open(filename).expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");
  let re = Regex::new(r"(\d+)").unwrap();

  content
    .split_terminator('\n')
    .map(|s| {
      let caps = re
        .captures_iter(s)
        .map(|i| i[1].parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
      let mut op = Op::new(caps[0], vec![caps[1], caps[2], caps[3]]);
      if caps.len() > 5 {
        op.register_states
          .push(vec![caps[4], caps[5], caps[6], caps[7]]);
        op.register_states
          .push(vec![caps[8], caps[9], caps[10], caps[11]]);
      }
      op
    })
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let mut op = Op::new(9, vec![2, 1, 2]);
    op.register_states.push(vec![3, 2, 1, 1]);
    op.register_states.push(vec![3, 2, 2, 1]);

    assert_eq!(find_potential_operations(&op), ["addi", "mulr", "seti"]);

    let ops = vec![op];
    assert_eq!(find_operations_count(&ops), 1);
  }
}
