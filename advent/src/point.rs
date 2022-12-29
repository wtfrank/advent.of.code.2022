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

