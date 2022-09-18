pub const DEFAULT_SEARCH_DIST: f64 = 150.0;
const STEP_SIZE: f64 = 10.0;

pub fn ray_march<'a>(position: linal::Vec3,
                 direction: linal::Vec3,
                 objects: &Vec<&'a dyn crate::object::Object>,
                 search_distance: f64) -> 
Option<(&'a dyn crate::object::Object, linal::Vec3)> {
    for i in 0..((search_distance/STEP_SIZE) as i32) {
        let point_position = position + direction * (i as f64) * STEP_SIZE;
        let point_prev_position = position + direction * (i as f64 - 1.0) * STEP_SIZE;
        for object in objects.iter() {
            if object.check_intersection(point_position) {
                return Some((*object, point_prev_position));
            }
        }
    }
    None
}
