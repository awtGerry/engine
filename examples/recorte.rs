use engine::graphics::window::Window;
use engine::algorithms::figures::draw_rectangle;
use engine::algorithms::circles::draw_circle_normal;

const MAX_RECTANGLE_X: f32 = 0.5;
const MIN_RECTANGLE_X: f32 = -0.5;
const MAX_RECTANGLE_Y: f32 = 0.5;
const MIN_RECTANGLE_Y: f32 = -0.5;

const CIRCLE_CENTER_X: f32 = 450.0;
const CIRCLE_CENTER_Y: f32 = 300.0;
const CIRCLE_RADIUS: f32 = 200.0;

fn main() {
    let mut window = Window::new(800, 600, "Recorte");
    window.init();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            draw_rectangle(MIN_RECTANGLE_X, MIN_RECTANGLE_X, MAX_RECTANGLE_Y, MAX_RECTANGLE_Y);
            gl::DrawArrays(gl::LINES, 0, 5);
            draw_circle_normal(CIRCLE_CENTER_X, CIRCLE_CENTER_Y, CIRCLE_RADIUS, -200.0);
        }
        window.update();
    }
}
