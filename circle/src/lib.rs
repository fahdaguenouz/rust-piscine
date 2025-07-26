use std::f64::consts::PI;
#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center :Point,
	pub radius :f64,
}

impl Circle {
   pub fn new(x:f64,y:f64,r:f64)->Self{
    Self{
        center:Point(x,y),
        radius:r,
    }
   }
   pub fn diameter(&self)->f64{
    let dia=self.radius;
    dia*2.0
    
   }
   pub fn area(&self)->f64{
    PI*(self.radius*self.radius)
   }
   pub fn intersect(&self,c2:Circle)->bool{
    let dis=self.center.distance(c2.center);
    let rad2=self.radius+c2.radius;
    rad2>=dis
   }
   
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64,pub f64);

impl Point {
    pub fn distance(&self,b:Point)->f64{
        ( ((self.0-b.0).powi(2)) + ((self.1-b.1).powi(2))).sqrt()
    }
}