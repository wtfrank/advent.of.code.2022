use crate::{Point,Point3,Dims,Dims3};

#[derive(Debug)]
enum Origin {
  TopLeft,
  TopRight,
  BottomRight,
  BottomLeft,
}

#[derive(Debug)]
pub struct TerrainMap<T:Copy+Default> {
  v: Vec<T>,
  origin: Origin,
  pub dims: Dims,
}

impl<T:Copy+Default> TerrainMap<T> {
  pub fn new(dims: Dims) -> TerrainMap<T> {
    let mut tm:TerrainMap<T> = TerrainMap{ v: Vec::new(), origin: Origin::TopLeft, dims };
    tm.v.resize_with(dims.width * dims.height, T::default);
    tm
  }

  pub fn get(&self, p: &Point) -> T {
    self.getc(p.x, p.y)
  }
  pub fn getc(&self, x: isize, y: isize) -> T {
    *self.v.get(self.coords_to_offset(x,y)).unwrap()
  }
  pub fn set(&mut self, p: &Point, val:T) {
    self.setc(p.x, p.y, val )
  }

  pub fn setc(&mut self, x: isize, y: isize, val: T) {
    let offset = self.coords_to_offset(x, y);
    self.v[offset] = val;
  }

  fn coords_to_offset(&self, x: isize, y: isize ) -> usize {
    assert!(x >= self.dims.minx);
    assert!(y >= self.dims.miny);
    assert!(x < self.dims.minx.saturating_add_unsigned(self.dims.width));
    assert!(y < self.dims.miny.saturating_add_unsigned(self.dims.height));

    let x1:usize = (x-self.dims.minx) as usize;
    let y1:usize = (y-self.dims.miny) as usize;
    let (x2,y2) = match self.origin {
      Origin::TopLeft => (x1, y1),
      Origin::TopRight => (y1,self.dims.height - x1 -1),
      Origin::BottomRight => (self.dims.width - x1 -1, self.dims.height - y1 -1),
      Origin::BottomLeft => (self.dims.width - y1 -1, x1),
    };

    //println!("x: {x} => {x2}, y: {y} => {y2}");
    y2 * self.dims.width + x2
  }


  // rotates the matrix 90 degrees clockwise
  // this is done in place so it's fast (accessors feel the pain)
  pub fn rotate_cw(&mut self) {
    assert_eq!(self.dims.width, self.dims.height);
    self.origin = match self.origin {
      Origin::TopLeft => Origin::TopRight,
      Origin::TopRight => Origin::BottomRight,
      Origin::BottomRight => Origin::BottomLeft,
      Origin::BottomLeft => Origin::TopLeft,
    }
  }

  // rotates the matrix 90 degrees anti-clockwise
  // this is done in place so it's fast (accessors feel the pain)
  pub fn rotate_acw(&mut self) {
    assert_eq!(self.dims.width, self.dims.height);
    self.origin = match self.origin {
      Origin::TopLeft => Origin::BottomLeft,
      Origin::BottomLeft => Origin::BottomRight,
      Origin::BottomRight => Origin::TopRight,
      Origin::TopRight => Origin::TopLeft,
    }
  }
}

#[derive(Debug)]
pub struct TerrainMap3<T:Copy+Default> {
  v: Vec<T>,
  pub dims: Dims3,
}

impl<T:Copy+Default> TerrainMap3<T> {
  pub fn new(dims: Dims3) -> TerrainMap3<T> {
    let mut tm:TerrainMap3<T> = TerrainMap3{ v: Vec::new(), dims };
      tm.v.resize_with(dims.width * dims.height * dims.depth, T::default);
    tm
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

    #[test]
    fn test_get_set() {
      let mut tm = TerrainMap::<usize>::new(Dims{width:5,height:5,..Default::default()});
      tm.set(&Point{x:1,y:2}, 15);
      assert!(tm.get(&Point{x:1, y:2}) == 15);
      tm.set(&Point{x:4,y:4}, 16);
      assert!(tm.get(&Point{x:4, y:4}) == 16);
      tm.rotate_cw();
      tm.set(&Point{x:2,y:3}, 17);
      assert!(tm.get(&Point{x:2, y:3}) == 17);
    }

    #[test]
    fn test_rotate_cw() {
      let mut tm = TerrainMap::<usize>::new(Dims{width: 2, height: 2, ..Default::default()});
      tm.setc(0,0,1);

      assert_eq!(tm.getc(0,0), 1);

      tm.rotate_cw();
      assert_eq!(tm.getc(0,0), 0);
      assert_eq!(tm.getc(1,0), 1);

      tm.rotate_cw();
      assert_eq!(tm.getc(1,0), 0);
      assert_eq!(tm.getc(1,1), 1);

      tm.rotate_cw();
      assert_eq!(tm.getc(1,1), 0);
      assert_eq!(tm.getc(0,1), 1);

      tm.rotate_cw();
      assert_eq!(tm.getc(0,1), 0);
      assert_eq!(tm.getc(0,0), 1);

   }
    #[test]
    fn test_rotate_acw() {
      let mut tm = TerrainMap::<usize>::new(Dims{width: 3, height: 3, ..Default::default()});
      tm.setc(0,0,1);

      assert_eq!(tm.getc(0,0), 1);

      tm.rotate_acw();
      assert_eq!(tm.getc(0,0), 0);
      assert_eq!(tm.getc(0,2), 1);

      tm.rotate_acw();
      assert_eq!(tm.getc(0,2), 0);
      assert_eq!(tm.getc(2,2), 1);

      tm.rotate_acw();
      assert_eq!(tm.getc(2,2), 0);
      assert_eq!(tm.getc(2,0), 1);

      tm.rotate_acw();
      assert_eq!(tm.getc(2,0), 0);
      assert_eq!(tm.getc(0,0), 1);
    }
}
 
