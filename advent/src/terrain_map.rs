use crate::{Point,Dims};

pub struct TerrainMap<T:Copy+Default> {
  v: Vec< Vec<T>>, //each inner vector is a horizontal row
  pub dims: Dims,
}

impl<T:Copy+Default> TerrainMap<T> {
  pub fn new(dims: Dims) -> TerrainMap<T> {
    let mut tm:TerrainMap<T> = TerrainMap{ v: Vec::new(), dims };
    for _ in 0..dims.height {
      tm.v.push(vec![T::default();dims.width]);
    }
    return tm;
  }

  pub fn get(&self, p: Point) -> T {
    assert!(p.x >= self.dims.minx);
    assert!(p.y >= self.dims.miny);
    assert!(p.x < self.dims.minx.saturating_add_unsigned(self.dims.width));
    assert!(p.y < self.dims.miny.saturating_add_unsigned(self.dims.height));

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
      let mut tm = TerrainMap::<usize>::new(Dims{width:5,height:19,..Default::default()});
      assert!(tm.dims.minx == 0);
      assert!(tm.dims.miny == 0);
      assert!(tm.dims.width == 5);
      assert!(tm.dims.height == 19);

      tm.set(Point{x:1,y:2}, 15);
      assert!(tm.get(Point{x:1, y:2}) == 15);
    }
}
 
