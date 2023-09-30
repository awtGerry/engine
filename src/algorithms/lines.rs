use crate::graphics::wrapper::{Vao, Buffer};

use crate::algorithms::pixel::draw_pixel;
use crate::algorithms::raw_attributes::set_vao_vbo;

pub fn draw_dda_line(x1: f32, y1: f32, x2: f32, y2: f32) {
    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);

    set_vao_vbo(&vao, &vbo, &[x1, y1, x2, y2], 2);

    let dx = x2 - x1;
    let dy = y2 - y1;

    let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };

    let x_inc = dx as f32 / steps as f32;
    let y_inc = dy as f32 / steps as f32;

    let mut x = x1 as f32;
    let mut y = y1 as f32;

    for _ in 0..steps as i32 {
        draw_pixel(x as i32, y as i32);
        unsafe {
            gl::DrawArrays(gl::LINES, 0, 2);
        }
        x += x_inc;
        y += y_inc;
    }
}

pub fn bresenham_line(x1: f32, y1: f32, x2: f32, y2: f32) {
    let cordenates: [f32; 4] = [x1, y1, x2, y2];

    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    set_vao_vbo(&vao, &vbo, &cordenates, 3);

    let mut dx = x2 - x1;
    let mut dy = y2 - y1;

    let mut x = x1 as f32;
    let mut y = y1 as f32;

    let mut p = 2.0 * dy - dx;

    let mut inc_x = 1.0;
    let mut inc_y = 1.0;

    if dx < 0.0 {
        inc_x.clone_from(&-1.0);
        dx = -dx;
    }

    if dy < 0.0 {
        inc_y.clone_from(&-1.0);
        dy = -dy;
    }

    draw_pixel(x as i32, y as i32);

    while x < x2 {
        if p >= 0.0 {
            draw_pixel(x as i32, y as i32);
            y += 1.0;
            p += 2.0 * dy - 2.0 * dx;
        } else {
            draw_pixel(x as i32, y as i32);
            p += 2.0 * dy;
        }
        x += 1.0;
    }
}

/* Same as draw_dda_line but filter the array to be smaller */
pub fn draw_small_line(x1: f32, y1: f32, x2: f32, y2: f32) {
    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);

    set_vao_vbo(&vao, &vbo, &[x1, y1, x2, y2], 2);

    unsafe {
        gl::DrawArrays(gl::LINES, 0, 2);
    }
}

/* This one is for the clock */
pub fn draw_polar_line(x1: f32, y1: f32, length: f32, angle: f32) {
    let angle = angle.to_radians();

    let x2 = x1 + length * angle.cos();
    let y2 = y1 - length * angle.sin();

    draw_small_line(x1, y1, x2, y2);
}
