// use crate::graphics::wrapper::{Vao, Buffer};
// use crate::algorithms::raw_attributes::set_vao_vbo;

use crate::algorithms::pixel::draw_pixel;
use crate::algorithms::lines::draw_dda_line;

#[allow(unused)]
pub fn draw_cricle_normal(xc: f32, yc: f32, r: f32) {
    let mut x = 0.0;
    let mut y = r;

    let mut d = 1.0 - r;

    while x < y {
        draw_pixel(xc as i32 + x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 + y as i32, yc as i32 + x as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 - y as i32, yc as i32 + x as i32);
        draw_pixel(xc as i32 + x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 + y as i32, yc as i32 - x as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 - y as i32, yc as i32 - x as i32);

        if d < 0.0 {
            d += 2.0 * x + 3.0;
        } else {
            d += 2.0 * (x - y) + 5.0;
            y -= 1.0;
        }
        x += 1.0;
    }
}

pub fn draw_circle_mid_point(xc: f32, yc: f32, r: f32) {
    let mut x = 0.0;
    let mut y = r;

    let mut d = 1.0 - r;

    while x < y {
        draw_pixel(xc as i32 + x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 + y as i32, yc as i32 + x as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 - y as i32, yc as i32 + x as i32);
        draw_pixel(xc as i32 + x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 + y as i32, yc as i32 - x as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 - y as i32, yc as i32 - x as i32);

        if d < 0.0 {
            d += 2.0 * x + 3.0;
        } else {
            d += 2.0 * (x - y) + 5.0;
            y -= 1.0;
        }
        x += 1.0;
    }
}

#[allow(unused)]
pub fn draw_polar_circle(x: f32, y: f32, xc: f32, yc: f32) {
    let mut theta: f32 = 0.0;
    let segments = 360.0;
    let r = ((x - xc).powi(2) + (y - yc).powi(2)).sqrt();
    
    for _ in 0..segments as i32 {
        let x = xc + r * theta.cos();
        let y = yc + r * theta.sin();
        draw_pixel(x as i32, y as i32);
        theta += 2.0 * std::f32::consts::PI / segments;
    }
}


// TODO: There is a bug in this function, it will draw and fill the circle but at
//       compile/start time it will have a weird behavior (like screen tearing), fix that.
#[allow(unused)]
pub fn fill_circle(xc: f32, yc: f32, r: f32) {
    let mut x = 0.0;
    let mut y = r;

    let mut d = 1.0 - r;

    let coord: [f32; 2] = [
        xc, yc,
    ];

    while x < y {
        draw_dda_line(xc + x, yc + y, xc - x, yc + y);
        draw_dda_line(xc + y, yc + x, xc - y, yc + x);
        draw_dda_line(xc + x, yc - y, xc - x, yc - y);
        draw_dda_line(xc + y, yc - x, xc - y, yc - x);

        if d < 0.0 {
            d += 2.0 * x + 3.0;
        } else {
            d += 2.0 * (x - y) + 5.0;
            y -= 1.0;
        }
        x += 1.0;
    }
}
