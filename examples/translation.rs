/* TRANSLATION
 * 
 * This is a test file for the translation algorithm.
 * It will move a figure from one point to another.
 *
*/

use engine::graphics::window::Window;
use engine::graphics::shader::Shader;
// use engine::algorithms::lines::draw_dda_line;

fn main() {
    let mut window = Window::new(800, 600, "Traslation");
    window.init();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.update();
    }

}
