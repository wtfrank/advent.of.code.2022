#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Interval {
  pub start: isize,
  pub length: usize,
}

impl std::fmt::Display for Interval {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "({}-{})",
      self.start,
      self.start.saturating_add_unsigned(self.length)
    )
  }
}

#[derive(Debug, PartialEq)]
pub enum Overlap {
  Less,    //both start and end are lower than the other's start
  Left,    //starts outside to the left, ends inside other
  Equal,   //start and end are identical
  Outside, //start is lower, end is higher than the other
  Inside,  //start is higher, end is lower than the other
  Right,   //starts inside other, ends outsides
  Greater, //start and end are higher than the other
}

impl Interval {
  // end is not part of the interval. Therefore start -> end makes a half-open interval
  pub fn end(&self) -> isize {
    self.start.checked_add_unsigned(self.length).unwrap()
  }

  pub fn cmp_overlap(&self, other: &Interval) -> Overlap {
    if self.end() <= other.start {
      Overlap::Less
    } else if self.start < other.start && self.end() > other.start && self.end() < other.end() {
      Overlap::Left
    } else if self.start == other.start && self.end() == other.end() {
      Overlap::Equal
    } else if self.start <= other.start && self.end() >= other.end() {
      Overlap::Outside
    } else if self.start >= other.start && self.end() <= other.end() {
      Overlap::Inside
    } else if self.start >= other.start && self.start < other.end() && self.end() > other.end() {
      Overlap::Right
    } else if self.start >= other.end() {
      Overlap::Greater
    } else {
      panic!("non-exhaustive if statement");
    }
  }

  pub fn overlaps(&self, other: &Interval) -> bool {
    !matches!(self.cmp_overlap(other), Overlap::Less | Overlap::Greater)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_overlap_less() {
    let i1 = Interval { start: 10, length: 20 };
    let i2 = Interval { start: 30, length: 1 };
    let i3 = Interval { start: 31, length: 1 };

    assert_eq!(i1.cmp_overlap(&i2), Overlap::Less);
    assert_eq!(i1.cmp_overlap(&i3), Overlap::Less);
  }
  #[test]
  fn test_overlap_left() {
    let i1 = Interval { start: 10, length: 20 };
    let i2 = Interval { start: 15, length: 100 };

    assert_eq!(i1.cmp_overlap(&i2), Overlap::Left);
  }
  #[test]
  fn test_overlap_outside() {
    let i1 = Interval { start: 10, length: 20 };
    let i2 = Interval { start: 15, length: 5 };

    assert_eq!(i1.cmp_overlap(&i2), Overlap::Outside);
  }
  #[test]
  fn test_overlap_equal() {
    let i1 = Interval { start: 10, length: 20 };
    let i2 = Interval { start: 10, length: 20 };

    assert_eq!(i1.cmp_overlap(&i2), Overlap::Equal);
  }
  #[test]
  fn test_overlap_inside() {
    let i1 = Interval { start: 10, length: 20 };
    let i2 = Interval { start: 5, length: 50 };

    assert_eq!(i1.cmp_overlap(&i2), Overlap::Inside);
  }
  #[test]
  fn test_overlap_right() {
    let i1 = Interval { start: 15, length: 50 };
    let i2 = Interval { start: 10, length: 20 };

    assert_eq!(i1.cmp_overlap(&i2), Overlap::Right);
  }
  #[test]
  fn test_overlap_greater() {
    let i1 = Interval { start: 10, length: 20 };
    let i2 = Interval { start: 1, length: 7 };

    assert_eq!(i1.cmp_overlap(&i2), Overlap::Greater);
  }
}
