use std::fs::File;
use std::io::prelude::*;

use std::collections::VecDeque;

fn process_cmd(tokens: &mut std::str::SplitWhitespace, current_dir_index: &mut usize, tree: &VecDeque<FsNode>) {
  let cmd = tokens.next().unwrap();
  if cmd == "ls" {
  } else if cmd == "cd" {
    let arg = tokens.next().unwrap();
    if arg == "/" {
      *current_dir_index = 0;
      return;
    } else if arg == ".." {
      let current_dir = tree.get(*current_dir_index).unwrap();
      *current_dir_index = current_dir.parent.unwrap();
      return;
    } else {
      //could be more efficient by starting at current_dir_index
      //but whatevs
      for node in tree.iter() {
        if node.size.is_none()
          && node.parent.is_some()
          && node.parent.unwrap() == *current_dir_index
          && node.name == arg
        {
          *current_dir_index = node.index;
          return;
        }
      }
      panic!("Couldn't find directory {arg}");
    }
  } else {
    panic!("unexpected cmd {cmd}");
  }
}

fn process_ls_output(tokens: &mut std::str::SplitWhitespace, current_dir_index: &usize, tree: &mut VecDeque<FsNode>) {
  /* first token is either "dir" or a size".
   * second token is file/dir name
   */
  let dir_or_size = tokens.next().unwrap();
  let name = tokens.next().unwrap();
  if dir_or_size == "dir" {
    tree.push_back(FsNode {
      name: String::from(name),
      index: tree.len(),
      parent: Some(*current_dir_index),
      size: None,
      cum_space: 0,
    });
  } else {
    let size: usize = dir_or_size.parse::<usize>().unwrap();
    tree.push_back(FsNode {
      name: String::from(name),
      index: tree.len(),
      parent: Some(*current_dir_index),
      size: Some(size),
      cum_space: 0,
    });
  }
}

struct FsNode {
  name: String,
  index: usize,
  parent: Option<usize>, //reference to index - None if root node
  size: Option<usize>,   //None if directory
  cum_space: usize,      //space consumed by children, only for dirs
}

fn parse_input(filename: &str, current_dir_index: &mut usize, tree: &mut VecDeque<FsNode>) {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  for l in contents.lines() {
    let mut tokens = l.split_whitespace();
    let first = tokens.next().unwrap();
    if first == "$" {
      process_cmd(&mut tokens, current_dir_index, tree);
    } else {
      let mut tokens = l.split_whitespace();
      process_ls_output(&mut tokens, current_dir_index, tree);
    }
  }
}

/* because we will have always seen every parent
 * directory before every child dir or file,
 * we can run backwards through the list
 * to accumulate sizes
 */
fn calc_usage(tree: &mut VecDeque<FsNode>) {
  for i in (0..tree.len()).rev() {
    let node = tree.get(i).unwrap();
    let space_used = match node.size {
      None => node.cum_space,
      Some(s) => s,
    };
    match node.parent {
      None => (),
      Some(p) => {
        let parent = tree.get_mut(p).unwrap();
        parent.cum_space += space_used;
      }
    }
  }
}

fn part1_total_usage(tree: &VecDeque<FsNode>) -> usize {
  let mut total = 0;
  for node in tree.iter() {
    if node.cum_space <= 100_000 {
      total += node.cum_space;
    }
  }
  total
}

fn part2_freeable_space(tree: &VecDeque<FsNode>) -> usize {
  const TOTAL_FS_SIZE: usize = 70_000_000;
  const MIN_SPACE_NEEDED: usize = 30_000_000;

  let free_space = TOTAL_FS_SIZE - tree.front().unwrap().cum_space;
  let needed_space = MIN_SPACE_NEEDED - free_space;
  println!("Needs {needed_space} more free space");

  let mut best_dir = TOTAL_FS_SIZE;
  for n in tree.iter() {
    if n.cum_space < best_dir && n.cum_space >= needed_space {
      best_dir = n.cum_space;
    }
  }
  if best_dir == TOTAL_FS_SIZE {
    panic!("No directory was suitable to delete");
  }
  best_dir
}

fn main() -> std::io::Result<()> {
  let mut v: VecDeque<FsNode> = VecDeque::new();
  v.push_back(FsNode {
    name: String::from("/"),
    index: 0,
    parent: None,
    size: None,
    cum_space: 0,
  });
  let mut current_dir_index = 0;

  parse_input("input7.txt", &mut current_dir_index, &mut v);

  calc_usage(&mut v);

  println!("total space: {}", part1_total_usage(&v));
  println!("freeable space: {}", part2_freeable_space(&v));

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sum() {
    let mut v: VecDeque<FsNode> = VecDeque::new();
    v.push_back(FsNode {
      name: String::from("/"),
      index: 0,
      parent: None,
      size: None,
      cum_space: 0,
    });
    let mut current_dir_index = 0;

    parse_input("testinput.txt", &mut current_dir_index, &mut v);

    calc_usage(&mut v);

    assert_eq!(part1_total_usage(&v), 95437);

    assert_eq!(part2_freeable_space(&v), 24933642);
  }
}
