use aoc_2015_day3::*;

use clap::Parser;

/// Day 3 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input3.txt");
  let answer1 = analyse_input1(&data);
  println!("answer: {answer1}");
  let answer2 = analyse_input2(&data);
  println!("answer2: {answer2:?}");
}
