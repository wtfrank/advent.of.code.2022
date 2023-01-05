use crate::{Point,Point3,Dims,Dims3};

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

  pub fn get(&self, p: &Point) -> T {
    assert!(p.x >= self.dims.minx);
    assert!(p.y >= self.dims.miny);
    assert!(p.x < self.dims.minx.saturating_add_unsigned(self.dims.width));
    assert!(p.y < self.dims.miny.saturating_add_unsigned(self.dims.height));

    let v = self.v.get(p.y as usize).unwrap();
    *v.get(p.x as usize).unwrap()
  }
  pub fn set(&mut self, p: &Point, val:T) {
    assert!(p.x >= 0);
    assert!(p.y >= 0);
    let v: &mut Vec<T> = self.v.get_mut(p.y as usize).unwrap();
    v[p.x as usize] = val;
  }
}

pub struct TerrainMap3<T:Copy+Default> {
  v: Vec<T>,
  pub dims: Dims3,
}

impl<T:Copy+Default> TerrainMap3<T> {
  pub fn new(dims: Dims3) -> TerrainMap3<T> {
    let mut tm:TerrainMap3<T> = TerrainMap3{ v: Vec::new(), dims };
      tm.v.resize_with(dims.width * dims.height * dims.depth, T::default);
    return tm;
  }

  fn point_to_offset(&self, p:&Point3) -> usize {
    assert!(p.x >= self.dims.minx);
    assert!(p.y >= self.dims.miny);
    assert!(p.z >= self.dims.minz);
    assert!(p.x < self.dims.minx.saturating_add_unsigned(self.dims.width));
    assert!(p.y < self.dims.miny.saturating_add_unsigned(self.dims.height));
    assert!(p.z < self.dims.minz.saturating_add_unsigned(self.dims.depth));

    ((p.z - self.dims.minz) * self.dims.height as isize * self.dims.width as isize +
    (p.y - self.dims.miny) * self.dims.width as isize +
    (p.x - self.dims.minx)) as usize
  }

  pub fn get(&self, p: &Point3) -> T {
    self.v[self.point_to_offset(p)]
  }

  pub fn set(&mut self, p: &Point3, val:T) {
    let offset = self.point_to_offset(p);
    self.v[offset] = val;
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

      tm.set(&Point{x:1,y:2}, 15);
      assert!(tm.get(&Point{x:1, y:2}) == 15);
    }
    #[test]
    fn test_new3() {
      let mut tm = TerrainMap3::<usize>::new(Dims3{width:5,height:19,depth:7,minx: -1, miny: -2, minz: -3});

      tm.set(&Point3{x:1,y:2,z:3}, 15);
      assert!(tm.get(&Point3{x:1, y:2, z:3}) == 15);
    }
}
 
