use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::ops::Range;

use advent::{Point};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
      let (ranges, beacons) = load_scan("testinput.txt", 20);
      println!("{ranges:?}");
      let spaces = sum_ranges(&ranges[10], &beacons, 10);
      assert_eq!(spaces, 26);
      let tf = calc_tuning_frequency(&ranges, 20);
      assert_eq!(tf, 56000011);
    }

    #[test]
    fn test_create_range1() {
      let r_opt = create_range(Point{x:0,y:0}, Point{x:1,y:0},1);
      assert!(r_opt.is_some());
      let r = r_opt.unwrap();
      assert_eq!(r.start, 0);
      assert_eq!(r.end, 1);
    }
    #[test]
    fn test_create_range2() {
      let r_opt = create_range(Point{x:0,y:0}, Point{x:1,y:1},1);
      assert!(r_opt.is_some());
      let r = r_opt.unwrap();
      assert_eq!(r.start, -1);
      assert_eq!(r.end, 2);
    }


    #[test]
    fn test_insert_at_pos_after() {
      let mut l = LinkedList::<Range<isize>>::new();
      l.push_back(Range{start:0,end:1});
      insert_at_pos( Range{start:2, end:3}, 1, &mut l);
      assert_eq!(l.len(), 2);
      let first = l.iter().next().unwrap();
      assert_eq!(first.start, 0);
    }

     #[test]
    fn test_insert_at_pos_before() {
      let mut l = LinkedList::<Range<isize>>::new();
      l.push_back(Range{start:0,end:1});
      insert_at_pos( Range{start:2, end:3}, 0, &mut l);
      assert_eq!(l.len(), 2);
      let first = l.iter().next().unwrap();
      assert_eq!(first.start, 2);
    }

    #[test]
    fn test_insert_after() {
      let mut l = LinkedList::<Range<isize>>::new();
      l.push_back(Range{start:0,end:1});
      insert_range( Range{start:2, end:3}, &mut l);
      assert_eq!(l.len(), 2);
      let first = l.iter().next().unwrap();
      assert_eq!(first.start, 0);
    }

    #[test]
    fn test_insert_after2() {
      let mut l = LinkedList::<Range<isize>>::new();
      l.push_back(Range{start:0,end:1});
      l.push_back(Range{start:4,end:9});
      insert_range( Range{start:2, end:3}, &mut l);
      assert_eq!(l.len(), 3);
      let mut i = l.iter();
      let first = i.next().unwrap();
      assert_eq!(first.start, 0);
      let second = i.next().unwrap();
      assert_eq!(second.start, 2);
    }

    #[test]
    fn test_insert_before() {
      let mut l = LinkedList::<Range<isize>>::new();
      l.push_back(Range{start:2,end:3});
      insert_range( Range{start:0, end:1}, &mut l);
      assert_eq!(l.len(), 2);
      let first = l.iter().next().unwrap();
      assert_eq!(first.start, 0);
    }

    #[test]
    fn test_insert_merge1() {
      let mut l = LinkedList::<Range<isize>>::new();
      l.push_back(Range{start:0,end:1});
      insert_range( Range{start:1, end:2}, &mut l);
      assert_eq!(l.len(), 1);
      let first = l.iter().next().unwrap();
      assert_eq!(first.start, 0);
      assert_eq!(first.end, 2);
    }

    #[test]
    fn test_insert_merge2() {
      let mut l = LinkedList::<Range<isize>>::new();
      l.push_back(Range{start:2,end:3});
      insert_range( Range{start:1, end:2}, &mut l);
      assert_eq!(l.len(), 1);
      let first = l.iter().next().unwrap();
      assert_eq!(first.start, 1);
      assert_eq!(first.end, 3);
    }

    #[test]
    fn test_insert_merge3() {
      let mut l = LinkedList::<Range<isize>>::new();
      l.push_back(Range{start:2,end:5});
      insert_range( Range{start:0, end:3}, &mut l);
      assert_eq!(l.len(), 1);
      let first = l.iter().next().unwrap();
      assert_eq!(first.start, 0);
      assert_eq!(first.end, 5);
    }

    #[test]
    fn test_insert_merge_overlap() {
      let mut l = LinkedList::<Range<isize>>::new();
      l.push_back(Range{start:2,end:5});
      l.push_back(Range{start:10,end:15});
      insert_range( Range{start:3, end:12}, &mut l);
      assert_eq!(l.len(), 1);
      let first = l.iter().next().unwrap();
      assert_eq!(first.start, 2);
      assert_eq!(first.end, 15);
    }

    #[test]
    fn test_insert_merge_overlap2() {
      let mut l = LinkedList::<Range<isize>>::new();
      l.push_back(Range{start:2,end:5});
      l.push_back(Range{start:10,end:15});
      l.push_back(Range{start:16,end:22});
      insert_range( Range{start:3, end:20}, &mut l);
      assert_eq!(l.len(), 1);
      let first = l.iter().next().unwrap();
      assert_eq!(first.start, 2);
      assert_eq!(first.end, 22);
    }


    #[test]
    fn test_sum_ranges() {
      let mut l = LinkedList::<Range<isize>>::new();
      insert_range( Range{start:0, end:3}, &mut l);
      insert_range( Range{start:5, end:7}, &mut l);
      assert_eq!(sum_ranges(&l, &HashSet::new(), 1), 5);
    }
}

fn insert_at_pos(range: Range<isize>, pos: usize, list: &mut LinkedList<Range<isize>>) {
  let mut list2 = list.split_off(pos);
  list.push_back(range);
  list.append(&mut list2);
}

fn insert_range(range: Range<isize>, list: &mut LinkedList<Range<isize>>){
  //println!("Inserting {range:?}");
  let mut pos = 0;
  for r in list.iter_mut() {
    if range.end < r.start {
      insert_at_pos(range, pos, list);
      return;
    }
    else if r.end < range.start { 
      pos += 1;
      continue;
    }
    else { //the ranges overlap (possibly more than once)
      let new_r = Range{start: isize::min(r.start, range.start), 
                        end: isize::max(r.end, range.end)};

      //ideally we would use the unstable cursor feature
      //for this list splicing!

      let mut after = list.split_off(pos);
      after.pop_front();
      list.append(&mut after);
      insert_range(new_r, list);
      return;
    }
  }

  //we have reached the end of the list without finding a range that overlaps or should be inserted
  //before
  list.push_back(range);
}

fn create_range(sensor: Point, beacon: Point, row: isize) -> Option<Range<isize>> {
  let max_dist = sensor.rectilinear_dist(&beacon);

  let row_dist = isize::abs(sensor.y - row);

  if row_dist > max_dist as isize {
    return None;
  }

  Some(Range{start: sensor.x - (max_dist as isize - row_dist ), end: sensor.x + 1 + (max_dist as isize - row_dist)})
}

fn sum_ranges(ranges: &LinkedList<Range<isize>>, beacons: &HashSet<Point>, row: isize) -> usize {
  let mut sum:usize = 0;
  for r in ranges {
    sum += (r.end-r.start) as usize;
    for beacon in beacons {
      if beacon.y == row && r.contains(&beacon.x) {
        println!("range {r:?} contains beacon {beacon}");
        sum -= 1;
      }
    }
  }
  sum
}

fn load_scan(filename: &str, rows: usize) -> 
    (Vec<LinkedList<Range<isize>>>, HashSet<Point>)
{
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut ranges = Vec::<LinkedList::<Range<isize>>>::new();
  for _ in 0..rows+1 { ranges.push( LinkedList::<Range<isize>>::new() ); };
  let mut sensors = HashSet::<Point>::new();
  let mut beacons = HashSet::<Point>::new();
  let lines = contents.lines();
  for line in lines {
    let (sx,sy,bx,by) = sscanf::sscanf!(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}", isize, isize, isize, isize).unwrap();

    let s = Point{x: sx, y: sy};
    let b = Point{x: bx, y: by};

    sensors.insert(s);
    beacons.insert(b);
    for row in 0..rows+1 {
      let range = create_range(s,b,row as isize);
      match range {
        None=>continue,
        Some(r) => insert_range(r, &mut ranges[row]),
      };
    }
  }
  (ranges, beacons)
}

fn calc_tuning_frequency( ranges: &Vec<LinkedList<Range<isize>>>, max:usize) -> usize{
  for y in 0..max+1 {
    let rs = ranges.get(y).unwrap();
    if rs.len() > 1 {
      println!("multiple ranges: {y} {}", rs.len());
      let mut i = rs.iter();
      let first = i.next().unwrap();
      let second = i.next().unwrap();
      println!("first ends at {}, second starts at {}", first.end -1, second.start);
      assert_eq!(first.end + 1, second.start);
      return 4_000_000 * (second.start as usize -1) + y;
    }
    else if rs.len() <= 0 {
      panic!("unexpectedly nothing in row {y}");
    }
    else {
      let r = &rs.front().unwrap();
      if r.start > 0 || r.end < max as isize+2 {
        println!("incomplete cover of row {y}");
      }
    }
  }
  0
}

fn main() -> std::io::Result<()> {
  let (ranges,beacons) = load_scan("input15.txt", 4_000_000);

  let spaces = sum_ranges(&ranges[2_000_000], &beacons, 2_000_000);
  println!("spaces in row 2,000,000 {spaces}");

  let tf = calc_tuning_frequency(&ranges, 4_000_000);
  println!("tuning freq: {tf}");

  Ok(())
}
