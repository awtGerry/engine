use crate::graphics::wrapper::{Vao, Buffer};

use crate::algorithms::pixel::draw_pixel;
use crate::algorithms::raw_attributes::set_vao_vbo;

pub fn draw_infinite_symbol(xc: f32, yc: f32)
{
    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);

    let mut theta = 0.0;

    let r = 50.0;

    // set_vao_vbo(&vao, &vbo, &[xc, yc], 2);

    while theta <= 2.0 * std::f32::consts::PI
    {
        let x = (r*theta.cos()) + 200.0;
        let y = ((r*theta.sin()*2.0)/2.0) + 100.0;

        draw_pixel(x as i32, y as i32);

        unsafe {
            gl::DrawArrays(gl::LINES, 0, 2);
        }

        theta += 0.01;
    }
}

pub fn hundred_points(x: f32, y: f32, size: f32)
{
    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);

    set_vao_vbo(&vao, &vbo, &[x, y], 2);

    for i in 0..100
    {
        let x1 = std::f32::consts::PI * i as f32/99.0;
        let y1 = size * x1.sin();
    }
}
