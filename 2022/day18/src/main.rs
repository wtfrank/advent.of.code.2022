use advent::Dims3;
use advent::Point3;
use advent::TerrainMap3;
use std::fs::File;
use std::io::Read;

fn load_points(filename: &str) -> Vec<Point3> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let points = contents
    .trim()
    .lines()
    .map(|l| l.split(',').map(|n| n.parse::<isize>().unwrap()).collect())
    .map(Point3::from_vec)
    .collect();

  points
}

#[allow(clippy::int_plus_one)]
fn find_adjacent(points: &Vec<Point3>) -> usize {
  let max = points.iter().map(|p| p.max()).max().unwrap();

  let mut x_group = Vec::new();
  let mut y_group = Vec::new();
  let mut z_group = Vec::new();
  for _ in 0..max + 1 {
    x_group.push(Vec::<&Point3>::new());
    y_group.push(Vec::<&Point3>::new());
    z_group.push(Vec::<&Point3>::new());
  }

  for p in points {
    x_group[p.x as usize].push(p);
    y_group[p.y as usize].push(p);
    z_group[p.z as usize].push(p);
  }

  let mut adjacent = 0;
  for p in points {
    if p.x - 1 >= 0 {
      for q in &x_group[p.x as usize - 1] {
        if p.y == q.y && p.z == q.z {
          adjacent += 1
        }
      }
    }
    if p.x as usize + 1 < x_group.len() {
      for q in &x_group[p.x as usize + 1] {
        if p.y == q.y && p.z == q.z {
          adjacent += 1
        }
      }
    }

    if p.y - 1 >= 0 {
      for q in &y_group[p.y as usize - 1] {
        if p.x == q.x && p.z == q.z {
          adjacent += 1
        }
      }
    }
    if p.y as usize + 1 < y_group.len() {
      for q in &y_group[p.y as usize + 1] {
        if p.x == q.x && p.z == q.z {
          adjacent += 1
        }
      }
    }

    if p.z - 1 >= 0 {
      for q in &z_group[p.z as usize - 1] {
        if p.y == q.y && p.x == q.x {
          adjacent += 1
        }
      }
    }
    if p.z as usize + 1 < z_group.len() {
      for q in &z_group[p.z as usize + 1] {
        if p.y == q.y && p.x == q.x {
          adjacent += 1
        }
      }
    }
  }

  points.len() * 6 - adjacent
}

#[derive(Default, Clone, Copy, PartialEq)]
enum PointVisit {
  #[default]
  Unvisited,
  Visited,
  Unreachable,
}

#[allow(clippy::int_plus_one)]
fn flood_fill(points: &Vec<Point3>) -> usize {
  let max = 1 + points.iter().map(|p| p.max()).max().unwrap() as usize;
  let mut visited = TerrainMap3::<PointVisit>::new(Dims3 {
    height: max,
    width: max,
    depth: max,
    ..Default::default()
  });

  for p in points {
    visited.set(p, PointVisit::Unreachable);
  }

  let mut queue: Vec<Point3> = Vec::new();
  queue.push(Point3 { x: 0, y: 0, z: 0 }); //fails if 0 0 0 is in the input!

  loop {
    match queue.pop() {
      None => break,
      Some(p) => match visited.get(&p) {
        PointVisit::Unreachable => continue,
        PointVisit::Visited => continue,
        PointVisit::Unvisited => {
          visited.set(&p, PointVisit::Visited);
          if p.x - 1 >= visited.dims.minx {
            queue.push(Point3 {
              x: p.x - 1,
              y: p.y,
              z: p.z,
            });
          }
          if p.x + 1 < visited.dims.width as isize {
            queue.push(Point3 {
              x: p.x + 1,
              y: p.y,
              z: p.z,
            });
          }

          if p.y - 1 >= visited.dims.miny {
            queue.push(Point3 {
              x: p.x,
              y: p.y - 1,
              z: p.z,
            });
          }
          if p.y + 1 < visited.dims.height as isize {
            queue.push(Point3 {
              x: p.x,
              y: p.y + 1,
              z: p.z,
            });
          }
          if p.z - 1 >= visited.dims.minz {
            queue.push(Point3 {
              x: p.x,
              y: p.y,
              z: p.z - 1,
            });
          }
          if p.z + 1 < visited.dims.depth as isize {
            queue.push(Point3 {
              x: p.x,
              y: p.y,
              z: p.z + 1,
            });
          }
        }
      },
    }
  }

  let mut interior = Vec::new();
  for x in 0..visited.dims.width {
    for y in 0..visited.dims.height {
      for z in 0..visited.dims.depth {
        let p = Point3 {
          x: x as isize,
          y: y as isize,
          z: z as isize,
        };
        if visited.get(&p) == PointVisit::Unvisited {
          println!("{p} is in interior");
          interior.push(p);
        }
      }
    }
  }

  println!("{} interior points", interior.len());
  let inner_adj = find_adjacent(&interior);
  let outer_adj = find_adjacent(points);
  println!(
    "inner {}, outer {}, diff {}",
    inner_adj,
    outer_adj,
    outer_adj - inner_adj
  );

  outer_adj - inner_adj
}

fn main() {
  let points = load_points("input18.txt");
  let num_adj = find_adjacent(&points);
  println!("{num_adj}");
  let ff = flood_fill(&points);
  println!("{ff}");

  //get list of points
  //make 3 indexes by dimension
  //adjacent pair is defined as 2 cubes were 2 dimensions are identical
  // and the other differs by +/-1
  //surface area = 6 * total cubes - 2*adjacent pair

  //complexity - once points are sorted, we have to, for each cube, do a lookup of cubes with +/-1
  //of same dimensions. can be constant if we do it in an array indexed by coord. Then we scan
  //through every cube in the array and check for matching other coords. then we do this 2 more
  //times for other coords.
  //
  //so its n^2 algorithm
  //
  //
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let points = load_points("testinput.txt");
    let num_adj = find_adjacent(&points);
    assert_eq!(num_adj, 64);
    let ff = flood_fill(&points);
    assert_eq!(ff, 58);
  }
}
