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

#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq,Default)]
pub struct Dims3 {
  pub minx:isize,
  pub miny:isize,
  pub minz:isize,
  pub width:usize,
  pub height:usize,
  pub depth:usize,
}
impl std::fmt::Display for Dims3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}-{}, {}-{}, {}-{})",
        self.minx, self.minx.saturating_add_unsigned(self.width)-1,
        self.miny, self.miny.saturating_add_unsigned(self.height)-1,
        self.minz, self.minz.saturating_add_unsigned(self.depth)-1)
  }
}
