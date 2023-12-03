
use std::fs::File;
use std::io::Read;
use clap::Parser;

use advent::{TerrainMap,Dims,Point};

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

/// Day 3 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value_t=false)]
   benchmark: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load1() {
      let schematic = load_data( "testinput.txt" );
      let schematic_score = analyse_schematic(&schematic);
      assert_eq!(schematic_score, 4361);
    }
    #[test]
    fn test_load2() {
      let schematic = load_data( "testinput.txt" );
      let schematic_score = analyse_schematic2(&schematic);
      assert_eq!(schematic_score, 467835);
    }

}

fn check_around( schematic: &TerrainMap::<char>, start: &Point, end: &Point ) -> bool {
  //check entire surrounding rectangle including the number which is redundant but simple to code
  let mut s = Point{x: start.x-1, y:start.y-1};
  if s.x < 0 {s.x = 0;}
  if s.y < 0 {s.y = 0;}

  let mut e = Point{x: end.x+1, y:end.y+1};
  if e.x >= schematic.dims.width as isize { e.x = e.x-1; }
  if e.y >= schematic.dims.height as isize { e.y = e.y-1; }

  let mut pos = Point{ x: s.x, y: s.y };

  while pos.y <= e.y {
    while pos.x <= e.x {
      let c = schematic.get(&pos);
      if c != '.' && ! c.is_numeric() {
        return true;
      }

      pos.x += 1;
    }
    pos.x = s.x;
    pos.y+=1;
  }
  false
}

fn check_around_for_gear( schematic: &TerrainMap::<char>, start: &Point, end: &Point ) -> (bool,Point) {
  //check entire surrounding rectangle including the number which is redundant but simple to code
  let mut s = Point{x: start.x-1, y:start.y-1};
  if s.x < 0 {s.x = 0;}
  if s.y < 0 {s.y = 0;}

  let mut e = Point{x: end.x+1, y:end.y+1};
  if e.x >= schematic.dims.width as isize { e.x = e.x-1; }
  if e.y >= schematic.dims.height as isize { e.y = e.y-1; }

  let mut pos = Point{ x: s.x, y: s.y };

  while pos.y <= e.y {
    while pos.x <= e.x {
      let c = schematic.get(&pos);
      if c == '*' {
        return (true, pos);
      }

      pos.x += 1;
    }
    pos.x = s.x;
    pos.y+=1;
  }
  (false, pos)
}

fn pos_in_hrange( pos: &Point, start: &Point, end: &Point ) -> bool {
  if start.y != end.y {
    //panic!("range not in same horizontal area");
    return false;
  }
  if pos.y != start.y { return false; }
  if pos.x < start.x || pos.x > end.x { return false; }
  true
}

fn check_around_for_number( schematic: &TerrainMap::<char>, gpos: &Point, start: &Point, end: &Point) -> (bool,Point) {
  //check entire surrounding rectangle including the number which is redundant but simple to code
  let mut s = Point{x: gpos.x-1, y:gpos.y-1};
  if s.x < 0 {s.x = 0;}
  if s.y < 0 {s.y = 0;}

  let mut e = Point{x: gpos.x+1, y:gpos.y+1};
  if e.x >= schematic.dims.width as isize { e.x = e.x-1; }
  if e.y >= schematic.dims.height as isize { e.y = e.y-1; }

  let mut pos = Point{ x: s.x, y: s.y };

  while pos.y <= e.y {
    while pos.x <= e.x {
      if !pos_in_hrange(&pos, start, end) {
        let c = schematic.get(&pos);
        if c.is_numeric() {
          return (true, pos);
        }
      }

      pos.x += 1;
    }
    pos.x = s.x;
    pos.y+=1;
  }
  (false, pos)
}


fn analyse_schematic2( schematic: &TerrainMap::<char> ) -> usize {
  //go through schematic starting from bottom right
  //find start/end pos of numbers, and value
  //then check around for gear
  //if found, then check around gear for number

  let mut pos = Point{x: 0, y: 0};
  let mut start: Point = Default::default();
  let mut end: Point = Default::default();
  let mut in_number = false;
  let mut cur_number = 0;

  let mut score:usize = 0;

  while pos.y < schematic.dims.height as isize {
    while pos.x < schematic.dims.width as isize {
      let c = schematic.get(&pos);

      if in_number {
        if c.is_numeric() {
          end = pos;
          cur_number = cur_number * 10 + c.to_digit(10).unwrap();

        }
        else {
          in_number = false;
          let (found, g) = check_around_for_gear(schematic, &start, &end);
          if found {
            let (found, np) = check_around_for_number(schematic, &g, &start, &end);
            if !found {
              println!("Found gear with only one adjacent: {start}, {end}");
            }
            else {
              let mut np = np;
              //find start of 2nd number
              loop {
                let test = Point{x:np.x-1, y:np.y};
                if test.x < 0 { break; }
                let c = schematic.get(&test);
                if !c.is_numeric() { break; }
                np = test;
              }

              let mut num2 = 0;
              loop {
                let c = schematic.get(&np);
                if c.is_numeric() {
                  num2 = num2 * 10 + c.to_digit(10).unwrap();
                  np.x += 1;
                  if np.x >= schematic.dims.width as isize { break; }  
                }
                else {
                  break;
                }
              }
              println!("Found gear {cur_number}*{num2}");
              score += cur_number as usize * num2 as usize;
            }
          }
        }

      }
      else {
        if c.is_numeric() {
          in_number = true;
          start = pos;
          end = pos;
          cur_number = c.to_digit(10).unwrap();
        }
      }

      pos.x += 1;
    }
    pos.y += 1;
    pos.x = 0;
  }

  //we match each pair twice
  score / 2
}
fn analyse_schematic( schematic: &TerrainMap::<char> ) -> usize {
  //go through schematic starting from bottom right
  //find start/end pos of numbers, and value
  //then check around for symbols (non period)

  let mut pos = Point{x: 0, y: 0};
  let mut start: Point = Default::default();
  let mut end: Point = Default::default();
  let mut in_number = false;
  let mut cur_number = 0;

  let mut score:usize = 0;

  while pos.y < schematic.dims.height as isize {
    while pos.x < schematic.dims.width as isize {
      let c = schematic.get(&pos);

      if in_number {
        if c.is_numeric() {
          end = pos;
          cur_number = cur_number * 10 + c.to_digit(10).unwrap();

        }
        else {
          in_number = false;
          if check_around(schematic, &start, &end) {
            score += cur_number as usize;
          }
        }

      }
      else {
        if c.is_numeric() {
          in_number = true;
          start = pos;
          end = pos;
          cur_number = c.to_digit(10).unwrap();
        }
      }

      pos.x += 1;
    }
    pos.y += 1;
    pos.x = 0;
  }
  score
}



fn determine_map_dims(data: &str) -> Dims {
  let mut width = 0;
  let mut height = 0;
  for l in data.lines() {
    height += 1;
    let w = l.len();
    if w > width {
      width = w;
    }
  }
  return Dims{width, height,..Default::default()};
}

fn load_data( filename: &str) -> TerrainMap::<char>
{
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut tm = TerrainMap::<char>::new( determine_map_dims( &contents ) );
  let mut pos = Point{x:0,y:0};

  for line in contents.lines() {
    for c in line.chars() {
      
      tm.set(&pos,c);
      pos.x += 1;
    }

    pos.x = 0;
    pos.y += 1;
  }

  tm
}


fn main() {
    env_logger::init();

    let args = Args::parse();
    if args.benchmark {
      return;
    }

    let schematic = load_data( "input3.txt" );
    let score = analyse_schematic(&schematic);
    println!("score: {score}");
    let score2 = analyse_schematic2(&schematic);
    println!("score2: {score2}");

}
