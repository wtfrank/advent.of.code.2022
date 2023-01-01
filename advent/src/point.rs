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
 
