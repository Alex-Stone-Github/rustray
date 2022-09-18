pub mod ray;
pub mod object;
pub mod render;
pub mod camera;
pub mod light;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const XFOV: f64 = std::f64::consts::FRAC_PI_2;
const YFOV: f64 = std::f64::consts::FRAC_PI_2;


fn main() {
    // sdl2
    let sdl_ctx = sdl2::init().unwrap();
    let sdl_video = sdl_ctx.video().unwrap();
    let window = sdl_video.window(
        "Ray Marching",
        WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut sdl_event_pump = sdl_ctx.event_pump().unwrap();

    // setup
    let sphere1 = object::Sphere::new(
        linal::Vec3::new(-50.0, 0.0, 40.0), 10.0,
        linal::Vec3::new(255.0, 0.0, 0.0));
    let boxer = object::Boxer::new(
        linal::Vec3::new(0.0, 0.0, 90.0), 
        linal::Vec3::new(10.0, 10.0, 10.0),
        linal::Vec3::new(0.0, 255.0, 0.0));
    let sphere2 = object::Sphere::new(
        linal::Vec3::new(50.0, 0.0, 140.0), 10.0,
        linal::Vec3::new(0.0, 0.0, 255.0));
    let objects: Vec<&dyn object::Object> = vec![
        &sphere1,
        &sphere2,
        &boxer,
    ];
    let mut camera = camera::Camera{
        position: linal::Vec3::new(0, 0, 0),
        horizontal_angle: 0.0,
        vertical_angle: 0.0
    };

    // loop
    'main: loop {
        for event in sdl_event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    break 'main;
                },
                _ => {}
            }
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
            canvas.clear();

            let frame_buffer = render::render(&camera, &objects);
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let index = (x * HEIGHT + y) as usize;
                    let color = frame_buffer[index];
                    canvas.set_draw_color(color);
                    let _ = canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
                }
            }
            canvas.present();
            camera.position.x -= 10.0;
        }
    }
}


