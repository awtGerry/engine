use crate::graphics::color::Color;

pub fn draw_pixel(x: i32, y: i32) {
    unsafe {
        gl::DrawArrays(gl::POINTS, x, y);
        gl::Enable(gl::SCISSOR_TEST);
        gl::Scissor(x, y, 1, 1);
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Disable(gl::SCISSOR_TEST);
        // gl::DrawArrays(gl::POINTS, 0, 1);
    }
}

pub fn draw_pixel_color(x: i32, y: i32, color: &Color)
{
    unsafe {
        // gl::DrawArrays(gl::POINTS, x, y);
        gl::Enable(gl::SCISSOR_TEST);
        gl::Scissor(x, y, 1, 1);
        gl::ClearColor(color.r, color.g, color.b, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::Disable(gl::SCISSOR_TEST);
        // gl::DrawArrays(gl::POINTS, 0, 1);
    }
}
