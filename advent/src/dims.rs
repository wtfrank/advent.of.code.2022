#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
pub struct Dims {
  pub width:usize,
  pub height:usize,
}

impl std::fmt::Display for Dims {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.width, self.height)
  }
}

