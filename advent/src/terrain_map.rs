use crate::{Point,Dims};

pub struct TerrainMap<T:Copy+Default> {
  v: Vec< Vec<T>>, //each inner vector is a horizontal row
  dims: Dims,
}

impl<T:Copy+Default> TerrainMap<T> {
  pub fn new(dims: Dims) -> TerrainMap<T> {
    let mut tm:TerrainMap<T> = TerrainMap{ v: Vec::new(), dims };
    for _ in 0..dims.height {
      tm.v.push(vec![T::default();dims.width]);
    }
    return tm;
  }

  pub fn dims(&self) -> Dims {
    self.dims
  }

  pub fn get(&self, p: Point) -> T {
    assert!(p.x >= 0);
    assert!(p.y >= 0);

    let v = self.v.get(p.y as usize).unwrap();
    *v.get(p.x as usize).unwrap()
  }
  pub fn set(&mut self, p: Point, val:T) {
    assert!(p.x >= 0);
    assert!(p.y >= 0);
    let v: &mut Vec<T> = self.v.get_mut(p.y as usize).unwrap();
    v[p.x as usize] = val;
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
      let mut tm = TerrainMap::<usize>::new(5,19);
      assert!(tm.width == 5);
      assert!(tm.height == 19);

      tm.set(Dims{x:1,y:2}, 15);
      assert!(tm.get(Dims{x:1, y:2}) == 15);
    }
}
 
