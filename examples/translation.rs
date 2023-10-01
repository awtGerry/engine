use engine::graphics::window::Window;
use engine::algorithms::figures::draw_rectangle;

fn main() {
    let mut window = Window::new(800, 600, "Translation");

    window.init();

    // Create matrix translation
    #[allow(unused)]
    let translation: [f32; 16] = [
        1.0, 0.0, 0.0, 0.0, // First column
        0.0, 1.0, 0.0, 0.0, // Second column
        0.0, 0.0, 1.0, 0.0, // Third column
        0.5, 0.5, 0.0, 1.0, // Fourth column
    ];
    
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            // gl::Clear(gl::COLOR_BUFFER_BIT);
            draw_rectangle(-0.5, -0.5, 0.5, 0.5);
            gl::DrawArrays(gl::LINES, 0, 5);
        }

        window.update();
    }
}
