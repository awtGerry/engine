pub fn draw_pixel(x: i32, y: i32) {
    unsafe {
        gl::DrawArrays(gl::POINTS, x, y);
        gl::Enable(gl::SCISSOR_TEST);
        gl::Scissor(x, y, 1, 1);
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Disable(gl::SCISSOR_TEST);
        gl::DrawArrays(gl::LINES, 0, 5);
    }
}
