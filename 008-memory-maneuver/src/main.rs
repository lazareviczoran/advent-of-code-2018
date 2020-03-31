use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let mut file = File::open("input.txt").expect("File not found");
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .expect("Failed to read input file");
  let data = content
    .split_terminator(' ')
    .map(|s| s.parse::<usize>().unwrap())
    .collect();

  let mut tree = HashMap::new();
  build_tree(&data, &mut 0, &mut 1, &mut tree);
  println!(
    "Day 8: Memory Maneuver part1 solution\n {}",
    calculate_metadata_sum(&tree, 1)
  );

  println!(
    "Day 8: Memory Maneuver part2 solution\n {}",
    calculate_node_value(&tree, 1)
  );
}

fn calculate_node_value(
  tree: &HashMap<usize, Node>,
  curr_node_id: usize,
) -> usize {
  let node = tree.get(&curr_node_id).unwrap();
  let mut sum = 0;
  if node.children.len() == 0 {
    sum = node.metadata_entries.iter().fold(sum, |acc, e| acc + e);
  } else {
    sum = node.metadata_entries.iter().fold(sum, |acc, e| {
      let mut val = 0;
      let item_index = *e;
      if item_index > 0 && item_index <= node.children.len() {
        val = calculate_node_value(tree, node.children[item_index - 1]);
      }
      acc + val
    });
  }
  sum
}

fn calculate_metadata_sum(
  tree: &HashMap<usize, Node>,
  curr_node_id: usize,
) -> usize {
  let node = tree.get(&curr_node_id).unwrap();
  let mut sum = node.metadata_entries.iter().fold(0, |acc, e| acc + e);
  sum = node
    .children
    .iter()
    .fold(sum, |acc, c| acc + calculate_metadata_sum(tree, *c));
  sum
}

fn build_tree(
  data: &Vec<usize>,
  curr_index: &mut usize,
  curr_node: &mut usize,
  tree: &mut HashMap<usize, Node>,
) {
  if *curr_index >= data.len() {
    return;
  }
  let mut node = Node::new(*curr_node);
  *curr_node += 1;
  let children_count = data[*curr_index];
  *curr_index += 1;

  let metadata_count = data[*curr_index];

  for _ in 0..children_count {
    *curr_index += 1;
    node.children.push(*curr_node);
    build_tree(data, curr_index, curr_node, tree);
  }
  for _ in 1..=metadata_count {
    *curr_index += 1;
    node.metadata_entries.push(data[*curr_index]);
  }
  tree.insert(node.id, node);
}

#[derive(Debug)]
struct Node {
  id: usize,
  children: Vec<usize>,
  metadata_entries: Vec<usize>,
}
impl Node {
  pub fn new(id: usize) -> Node {
    Node {
      id,
      children: Vec::new(),
      metadata_entries: Vec::new(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn part1_test() {
    let mut tree = HashMap::new();
    build_tree(
      &vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2],
      &mut 0,
      &mut 1,
      &mut tree,
    );
    assert_eq!(calculate_metadata_sum(&tree, 1), 138);
  }

  #[test]
  fn part2_test() {
    let mut tree = HashMap::new();
    build_tree(
      &vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2],
      &mut 0,
      &mut 1,
      &mut tree,
    );
    assert_eq!(calculate_node_value(&tree, 1), 66);
  }
}
