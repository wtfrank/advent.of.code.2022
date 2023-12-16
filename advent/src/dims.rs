use crate::{Point, Point3};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Dims {
  pub minx: isize,
  pub miny: isize,
  pub width: usize,
  pub height: usize,
}
impl std::fmt::Display for Dims {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "({}-{}, {}-{})",
      self.minx,
      self.minx.saturating_add_unsigned(self.width) - 1,
      self.miny,
      self.miny.saturating_add_unsigned(self.height) - 1
    )
  }
}

impl Dims {
  pub fn containsc(&self, x: isize, y: isize) -> bool {
    x >= self.minx
      && y >= self.miny
      && x < self.minx.saturating_add_unsigned(self.width)
      && y < self.miny.saturating_add_unsigned(self.height)
  }
  pub fn contains(&self, point: &Point) -> bool {
    self.containsc(point.x, point.y)
  }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Dims3 {
  pub minx: isize,
  pub miny: isize,
  pub minz: isize,
  pub width: usize,
  pub height: usize,
  pub depth: usize,
}
impl std::fmt::Display for Dims3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "({}-{}, {}-{}, {}-{})",
      self.minx,
      self.minx.saturating_add_unsigned(self.width) - 1,
      self.miny,
      self.miny.saturating_add_unsigned(self.height) - 1,
      self.minz,
      self.minz.saturating_add_unsigned(self.depth) - 1
    )
  }
}

impl Dims3 {
  pub fn containsc(&self, x: isize, y: isize, z: isize) -> bool {
    x >= self.minx
      && y >= self.miny
      && z >= self.minz
      && x < self.minx.saturating_add_unsigned(self.width)
      && y < self.miny.saturating_add_unsigned(self.height)
      && z < self.minz.saturating_add_unsigned(self.depth)
  }
  pub fn contains(&self, point: &Point3) -> bool {
    self.containsc(point.x, point.y, point.z)
  }
}

/* a utility function which takes as input a
 * multi-line string and calculates its
 * dimensions
 */
pub fn determine_map_dims(data: &str) -> Dims {
  let mut width = 0;
  let mut height = 0;
  for l in data.lines() {
    height += 1;
    let w = l.len();
    if w > width {
      width = w;
    }
  }

  Dims {
    width,
    height,
    ..Default::default()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_contains() {
    let dim = Dims {
      minx: -2,
      miny: -2,
      width: 5,
      height: 5,
    };

    assert!(dim.containsc(0, 0));
    assert!(dim.containsc(-1, -1));
    assert!(dim.containsc(-2, -2));

    assert!(!dim.containsc(-3, -3));
    assert!(dim.containsc(2, 2));
    assert!(!dim.containsc(3, 3));
  }

  #[test]
  fn test_contains3() {
    let dim = Dims3 {
      minx: -2,
      miny: -2,
      minz: -2,
      width: 5,
      height: 5,
      depth: 5,
    };

    assert!(dim.containsc(0, 0, 0));
    assert!(dim.containsc(-1, -1, -1));
    assert!(dim.containsc(-2, -2, -2));

    assert!(!dim.containsc(-3, -3, -3));
    assert!(dim.containsc(2, 2, 2));
    assert!(!dim.containsc(3, 3, 3));
  }
}
