/* TRANSLATION
 * 
 * This is a test file for the translation algorithm.
 * It will move a figure from one point to another.
 *
*/

use engine::algorithms::figures::fill_rectangle_inundation;
use engine::graphics::window::Window;
use engine::algorithms::transformations::*;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 720;

fn main() {
    let mut window = Window::new(WIDTH, HEIGHT, "Traslation");
    window.init();

    let mut x = 600.0;
    let mut y = 360.0;

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            /* if x > WIDTH as f32 {
                x = 0.0;
            }
            (x, y) = translate(x, y, 1.0, 0.0);
            fill_rectangle_inundation(x, y, x+50.0, y+50.0); */
        }
        window.update();
    }

}
