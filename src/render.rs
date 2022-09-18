use crate::{WIDTH, HEIGHT, XFOV, YFOV};

fn map<T: Copy + std::ops::Sub<Output=T> + std::ops::Div<Output=T> + std::ops::Mul<Output=T> + std::ops::Add<Output=T>>
(value: T, x: T, y: T, a: T, b: T) -> T {
    let percent = (value - x) / (y - x);
    percent * (b - a) + a
}

pub fn render(camera: &crate::camera::Camera,
              objects: &Vec<&dyn crate::object::Object>) -> Vec<sdl2::pixels::Color> {
    let mut frame_buffer = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let horizontal_angle_offset = map::<f64>(x as f64, 0.0, WIDTH as f64, -XFOV/2.0, XFOV/2.0);
            let vertical_angle_offset = map::<f64>(y as f64, 0.0, HEIGHT as f64, -YFOV/2.0, YFOV/2.0);
            
            // I think this makes sense
            let direction = linal::Vec3::new(
                (camera.horizontal_angle + horizontal_angle_offset).sin(),
                (camera.vertical_angle + vertical_angle_offset).sin(),
                (camera.horizontal_angle + horizontal_angle_offset).cos());

            if let Some((collision, point)) = crate::ray::ray_march(
                camera.position, direction, objects, crate::ray::DEFAULT_SEARCH_DIST) {
                let color = collision.get_color();
                frame_buffer.push(sdl2::pixels::Color::RGB(color.x as u8, color.y as u8, color.z as u8));
            } else {
                frame_buffer.push(sdl2::pixels::Color::RGB(100, 100, 100));
            }
        }
    }
    frame_buffer
}
