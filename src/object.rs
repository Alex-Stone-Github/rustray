pub trait Object {
    fn check_intersection(&self, point: linal::Vec3) -> bool;
    fn get_color(&self) -> linal::Vec3;
}

pub struct Boxer {
    position: linal::Vec3,
    size: linal::Vec3,
    color: linal::Vec3,
}
impl Object for Boxer {
    fn check_intersection(&self, point: linal::Vec3) -> bool {
        if (point.x - self.position.x).abs() < self.size.x/2.0 {
            if (point.y - self.position.y).abs() < self.size.y/2.0 {
                if (point.z - self.position.z).abs() < self.size.z/2.0 {
                    return true;
                }
            }
        }
        false
    }
    fn get_color(&self) -> linal::Vec3 {
        self.color
    }
}
impl Boxer {
    pub fn new(position: linal::Vec3, size: linal::Vec3, color: linal::Vec3) -> Self {
        Self {
            position,
            size,
            color
        }
    }
}


pub struct Sphere {
    position: linal::Vec3,
    radius: f64,
    color: linal::Vec3,
}
impl Object for Sphere {
    fn check_intersection(&self, point: linal::Vec3) -> bool {
        let dist = ((point.x - self.position.x).powf(2.0) + 
            (point.y - self.position.y).powf(2.0) + 
            (point.z - self.position.z).powf(2.0)).sqrt();
        dist < self.radius
    }
    fn get_color(&self) -> linal::Vec3 {
        self.color
    }
}
impl Sphere {
    pub fn new(position: linal::Vec3, radius: f64, color: linal::Vec3) -> Self {
        Self {
            position,
            radius,
            color
        }
    }
}
