use std::fs::File;
use std::io::prelude::*;

use advent::{Dims, Point, TerrainMap};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rocks() {
    let mut rocks = init_rocks();
    let r5 = rocks.pop().unwrap();
    assert_eq!(r5.dims.width, 2);
    assert_eq!(r5.dims.height, 2);

    let r4 = rocks.pop().unwrap();
    assert_eq!(r4.dims.width, 1);
    assert_eq!(r4.dims.height, 4);

    let r3 = rocks.pop().unwrap();
    assert_eq!(r3.dims.width, 3);
    assert_eq!(r3.dims.height, 3);

    let r2 = rocks.pop().unwrap();
    assert_eq!(r2.dims.width, 3);
    assert_eq!(r2.dims.height, 3);

    let r1 = rocks.pop().unwrap();
    assert_eq!(r1.dims.width, 4);
    assert_eq!(r1.dims.height, 1);
  }

  #[test]
  fn test_compare() {
    let jets = load_jets("testinput.txt");
    let (_, height) = drop_rocks(&jets, 2022);
    assert_eq!(height, 3068);
  }
  #[test]
  #[ignore]
  fn test_compare_smart() {
    let jets = load_jets("testinput.txt");
    let height = drop_rocks_smart(&jets, 1000000000000);
    assert_eq!(height, 1514285714288);
  }

  #[test]
  #[ignore]
  fn test_compare_smart2() {
    let jets = load_jets("input17.txt");

    for i in 3450..4000 {
      let (_, h1) = drop_rocks(&jets, i);
      let h2 = drop_rocks_smart(&jets, i);
      assert_eq!(h1, h2);
    }
  }

  #[test]
  fn test_single_left() {
    let jets = vec![Jet::Left];
    let (column, height) = drop_rocks(&jets, 1);
    assert_eq!(column.get(&Point { x: 0, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 1, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 2, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 3, y: 0 }), Material::Rock);
    assert_eq!(height, 1);
  }
  #[test]
  fn test_single_right() {
    let jets = vec![Jet::Right];
    let (column, height) = drop_rocks(&jets, 1);
    assert_eq!(column.get(&Point { x: 0, y: 0 }), Material::Air);
    assert_eq!(column.get(&Point { x: 1, y: 0 }), Material::Air);
    assert_eq!(column.get(&Point { x: 2, y: 0 }), Material::Air);
    assert_eq!(column.get(&Point { x: 3, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 4, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 5, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 6, y: 0 }), Material::Rock);
    assert_eq!(height, 1);
  }
  #[test]
  fn test_double() {
    let jets = vec![Jet::Left];
    let (column, height) = drop_rocks(&jets, 2);
    assert_eq!(column.get(&Point { x: 0, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 1, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 2, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 3, y: 0 }), Material::Rock);

    assert_eq!(column.get(&Point { x: 0, y: 1 }), Material::Air);
    assert_eq!(column.get(&Point { x: 1, y: 1 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 2, y: 1 }), Material::Air);
    assert_eq!(column.get(&Point { x: 3, y: 1 }), Material::Air);

    assert_eq!(column.get(&Point { x: 0, y: 2 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 1, y: 2 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 2, y: 2 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 3, y: 2 }), Material::Air);

    assert_eq!(column.get(&Point { x: 0, y: 3 }), Material::Air);
    assert_eq!(column.get(&Point { x: 1, y: 3 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 2, y: 3 }), Material::Air);
    assert_eq!(column.get(&Point { x: 3, y: 3 }), Material::Air);

    assert_eq!(height, 4);
  }
  #[test]
  fn test_triple() {
    let jets = vec![Jet::Left];
    let (column, height) = drop_rocks(&jets, 3);

    visualise_column(&column, 8);

    assert_eq!(column.get(&Point { x: 0, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 1, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 2, y: 0 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 3, y: 0 }), Material::Rock);

    assert_eq!(column.get(&Point { x: 0, y: 1 }), Material::Air);
    assert_eq!(column.get(&Point { x: 1, y: 1 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 2, y: 1 }), Material::Air);
    assert_eq!(column.get(&Point { x: 3, y: 1 }), Material::Air);

    assert_eq!(column.get(&Point { x: 0, y: 2 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 1, y: 2 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 2, y: 2 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 3, y: 2 }), Material::Air);

    assert_eq!(column.get(&Point { x: 0, y: 3 }), Material::Air);
    assert_eq!(column.get(&Point { x: 1, y: 3 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 2, y: 3 }), Material::Air);
    assert_eq!(column.get(&Point { x: 3, y: 3 }), Material::Air);

    assert_eq!(column.get(&Point { x: 0, y: 4 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 1, y: 4 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 2, y: 4 }), Material::Rock);
    assert_eq!(column.get(&Point { x: 3, y: 4 }), Material::Air);

    assert_eq!(height, 7);
  }
}

enum Jet {
  Left,
  Right,
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
enum Material {
  #[default]
  Air,
  Rock,
}

fn visualise_line(column: &TerrainMap<Material>, y: usize) {
  let mut line = String::new();
  for x in 0..column.dims.width {
    line.push(
      match column.get(&Point {
        x: x as isize,
        y: y as isize,
      }) {
        Material::Air => '.',
        Material::Rock => '#',
      },
    );
  }
  println!("{line}");
}

#[cfg(test)]
fn visualise_column(column: &TerrainMap<Material>, max_height: usize) {
  for mut y in 0..max_height {
    y = max_height - y - 1;
    visualise_line(column, y);
  }
}

fn load_shape(shape: &str) -> TerrainMap<Material> {
  let mut y = 0;
  let mut x = 0;

  for line in shape.lines() {
    y += 1;
    x = line.len();
  }

  let mut sprite = TerrainMap::new(Dims {
    width: x,
    height: y,
    ..Default::default()
  });

  let mut y = (y - 1) as isize;
  for line in shape.lines() {
    for (x, c) in line.chars().enumerate() {
      let mat: Material = match c {
        '#' => Material::Rock,
        '.' => Material::Air,
        _ => panic!("unexpected char"),
      };
      sprite.set(&Point { x: x as isize, y }, mat);
    }
    y -= 1;
  }

  sprite
}

fn init_rocks() -> Vec<TerrainMap<Material>> {
  let v = vec![
    load_shape("####"),
    load_shape(
      ".#.\n\
           ###\n\
           .#.",
    ),
    load_shape(
      "..#\n\
           ..#\n\
           ###",
    ),
    load_shape(
      "#\n\
           #\n\
           #\n\
           #",
    ),
    load_shape(
      "##\n\
           ##",
    ),
  ];

  v
}

fn load_jets(filename: &str) -> Vec<Jet> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut jets = Vec::<Jet>::new();
  for c in contents.chars() {
    match c {
      '>' => jets.push(Jet::Right),
      '<' => jets.push(Jet::Left),
      '\n' => (),
      _ => panic!("unexpected jet direction: '{c}'"),
    }
  }

  jets
}

fn jet_rock(rock: &TerrainMap<Material>, rock_x: &mut isize, rock_y: isize, jet: &Jet, column: &TerrainMap<Material>) {
  let new_rock_x;
  match *jet {
    Jet::Left => {
      if *rock_x == 0 {
        return;
      } else {
        new_rock_x = *rock_x - 1
      }
    }
    Jet::Right => {
      if *rock_x + rock.dims.width as isize >= column.dims.width as isize {
        return;
      } else {
        new_rock_x = *rock_x + 1
      }
    }
  }

  //now check for intersection between rock and existing column terrain
  for y in 0..rock.dims.height as isize {
    for x in 0..rock.dims.width as isize {
      if rock.get(&Point { x, y }) != Material::Rock {
        continue;
      }
      //println!("{new_rock_x}, {rock_y}. {x},{y}.");
      if column.get(&Point {
        x: new_rock_x + x,
        y: rock_y + y,
      }) == Material::Rock
      {
        return;
      }
    }
  }
  *rock_x = new_rock_x;
}

fn land_rock(
  rock: &TerrainMap<Material>,
  rock_x: isize,
  rock_y: isize,
  column: &mut TerrainMap<Material>,
  highest_rock: &mut isize,
) {
  for y in 0..rock.dims.height as isize {
    for x in 0..rock.dims.width as isize {
      if rock.get(&Point { x, y }) != Material::Rock {
        continue;
      }
      column.set(
        &Point {
          x: rock_x + x,
          y: rock_y + y,
        },
        Material::Rock,
      );
    }
  }
  if *highest_rock < rock_y + rock.dims.height as isize {
    *highest_rock = rock_y + rock.dims.height as isize;
  }
}

// @return true if the rock landed
fn drop_rock(
  rock: &TerrainMap<Material>,
  rock_x: isize,
  rock_y: &mut isize,
  column: &mut TerrainMap<Material>,
  highest_rock: &mut isize,
) -> bool {
  let new_rock_y = *rock_y - 1;
  if new_rock_y < 0 {
    land_rock(rock, rock_x, *rock_y, column, highest_rock);
    return true;
  }
  //now check for intersection between rock and existing column terrain
  for y in 0..rock.dims.height as isize {
    for x in 0..rock.dims.width as isize {
      if rock.get(&Point { x, y }) != Material::Rock {
        continue;
      }
      if column.get(&Point {
        x: rock_x + x,
        y: new_rock_y + y,
      }) == Material::Rock
      {
        land_rock(rock, rock_x, *rock_y, column, highest_rock);
        return true;
      }
    }
  }

  *rock_y = new_rock_y;
  false
}

/* the rocks and jets cycle. we maybe can exploit this to
 * limit the simulation work.
 * if the top of the pile looked exactly the same at the time
 * that the jets had looped round to the start, and this coincided
 * with us releasing the same block, then the outcome would be
 * the same bar some linear offset.
 *
 * From inspection, it appears that each time it goes through all the jets, it has used 1725 rocks and
 * the height has increased by 2694.
 * It's not obvious that there have to be cycles, because even though jets/rocks cycle, the number
 * of locations that each rock falls differs depending on the terrain underneath.
 * We could write some code that detects cycles automatically, but given that we've got fairly high
 * confidence in a consistent cycle through manual log inspection, we'll just hardcode what we've
 * found, for speed.
 *
 * I would love to know if there's a mathematical proof that there will always be a
 * cycle in situations like this.
 *
 */
fn drop_rocks_smart(jets: &Vec<Jet>, count: usize) -> isize {
  let _total_jets = jets.len();

  let block_cycle = 1725; //after this number of blocks it cycles
  let cycle_height_increase = 2694;

  if count < 2 * block_cycle {
    let (_, height) = drop_rocks(jets, count);
    return height;
  }

  //We'll simulate a small amount for real and calcualate the rest

  let rem = count % block_cycle;

  let iters = count / block_cycle - 1;

  let (_, mut height) = drop_rocks(jets, block_cycle + rem);

  height += iters as isize * cycle_height_increase;

  height
}

/*
fn whole_row_solid( y: isize, col: &TerrainMap<Material>) -> bool {
  for x in 0..col.dims.width {
    if col.get(&Point{x:x as isize,y}) == Material::Air { return false; }

  }
  return true;
}
*/

fn drop_rocks(jets: &Vec<Jet>, count: usize) -> (TerrainMap<Material>, isize) {
  //y is 0 at bottom
  let mut column = TerrainMap::<Material>::new(Dims {
    width: 7,
    height: 3 + 3 * count,
    ..Default::default()
  });
  let rocks = init_rocks();
  let jet_len = jets.len();
  let rock_len = rocks.len();
  println!("{jet_len} jet seq, {rock_len} rocks.");
  let mut rocks = rocks.iter().cycle();
  let mut jets = jets.iter().cycle();

  let mut highest_rock = 0;
  let mut num_jets = 0;
  for _num_rocks in 0..count {
    let rock = rocks.next().unwrap();
    let mut rock_x = 2;
    let mut rock_y = highest_rock + 3;

    loop {
      //jet push
      let jet = jets.next().unwrap();
      jet_rock(rock, &mut rock_x, rock_y, jet, &column);
      num_jets += 1;
      //move down
      // if can't move down, update highest_rock, and break
      if drop_rock(rock, rock_x, &mut rock_y, &mut column, &mut highest_rock) {
        //check for a solid line all the way across
        /*
        for ly in 0..rock.dims.height {
          if whole_row_solid(rock_y + ly as isize, &column) {
            println!("Solid row at rock {}+{}({}), jet {}({}), highest {}", num_rocks % rock_len, ly, num_rocks, num_jets % jet_len, num_jets, highest_rock);
          }
        }*/
        break;
      }
    }

    //println!("{}", num_jets % jet_len);
    if num_jets % jet_len == 0
    //    && num_rocks % rock_len == 0
    {
      visualise_line(&column, highest_rock as usize);
    }
  }
  println!("total jets: {num_jets}");

  (column, highest_rock)
}

fn main() {
  let jets = load_jets("input17.txt");
  let (_col, height) = drop_rocks(&jets, 2022);
  println!("{height}");
  //visualise_column(&col, 100);

  //this is an absurd number of iterations and memory usage
  let height = drop_rocks_smart(&jets, 1_000_000_000_000);
  //let (_,height) = drop_rocks(&jets, 300_000);
  println!("{height}");
}
