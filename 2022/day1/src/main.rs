use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;


    let mut totals:Vec<i32> = Vec::new();
    let mut highest:i32 = -1;

    let mut cur:i32 = 0;
    for line in contents.lines() {
      if line.is_empty() {
        if cur > highest {
          println!("new elf with most calories so far: {cur}");
          highest = cur;
        }
        else {
          println!("pathetic elf");
        }
        totals.push(cur);
        cur = 0;
      }
      else {
        cur += line.trim().parse::<i32>().expect("number missing on line!");
      }
      //println!("{}", line);
    }

    totals.sort();
    println!("Best elf has {highest} calories");
    let mut top3 = 0;
    for t in totals[totals.len()-3..totals.len()].iter() {
      println!("{t}" );
      top3 += t;
    }
    println!("top 3 total = {top3}");
    Ok(())
}
