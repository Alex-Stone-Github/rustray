
pub trait Light {
    fn is_visible_to_point(&self, point: linal::Vec3, objects: &Vec<&dyn crate::object::Object>) -> bool;
}

pub struct DirectionalLight(linal::Vec3);
impl Light for DirectionalLight {
    fn is_visible_to_point(&self, point: linal::Vec3, objects: &Vec<&dyn crate::object::Object>) -> bool {
        let ray_direction = -self.0;
        if let Some(_) = crate::ray::ray_march(point, ray_direction, objects, 0.0) {
            return false;
        }
        true
    }
}
