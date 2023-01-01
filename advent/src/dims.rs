#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq,Default)]
pub struct Dims {
  pub minx:isize,
  pub miny:isize,
  pub width:usize,
  pub height:usize,
}
impl std::fmt::Display for Dims {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}-{}, {}-{})", self.minx, self.minx.saturating_add_unsigned(self.width)-1, self.miny, self.miny.saturating_add_unsigned(self.height)-1)
  }
}

