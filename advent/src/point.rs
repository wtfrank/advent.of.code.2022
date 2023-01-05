#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq,Default)]
pub struct Point {
  pub x:isize,
  pub y:isize,
}

impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl Point {
  //! Rectilinear distance is also known as L1, taxicab or manhattan distance
  pub fn rectilinear_dist(&self, p:&Point) -> usize {
      (isize::abs(self.x - p.x) + isize::abs(self.y-p.y)) as usize
  }
}

pub struct Point3 {
  pub x:isize,
  pub y:isize,
  pub z:isize,
}

impl std::fmt::Display for Point3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

impl Point3 {
  //! Rectilinear distance is also known as L1, taxicab or manhattan distance
  pub fn rectilinear_dist(&self, p:&Point3) -> usize {
      (isize::abs(self.x - p.x) + isize::abs(self.y-p.y) + isize::abs(self.z - p.z) ) as usize
  }

  pub fn from_vec(v: Vec<isize>) -> Point3 {
    assert_eq!(v.len(), 3);
    Point3{ x: v[0], y: v[1], z: v[2] }
  }

  pub fn max(&self) -> isize {
    std::cmp::max(std::cmp::max(self.x, self.y),self.z)
  }
}


#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_dist() {
      let a = Point{x:0,y:0};
      let b = Point{x:1,y:0};
      let c = Point{x:-1,y:1};

      assert_eq!(a.rectilinear_dist(&a), 0);
      assert_eq!(a.rectilinear_dist(&b), 1);
      assert_eq!(a.rectilinear_dist(&c), 2);
      assert_eq!(a.rectilinear_dist(&c), c.rectilinear_dist(&a));
    }
}
 
