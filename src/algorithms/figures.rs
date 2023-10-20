use crate::graphics::color::Color;
use crate::graphics::wrapper::{Vao, Buffer};
use crate::algorithms::pixel::{draw_pixel, draw_pixel_color};
use crate::algorithms::lines::draw_dda_line;

use std::ptr;

/* Collection of figures that may use (or not) the algorithms
 * implemented in the algorithms module.
 */
pub fn draw_rectangle(x1: f32, y1: f32, x2: f32, y2: f32)
{
    draw_dda_line(x1, y1, x2, y1);
    draw_dda_line(x2, y1, x2, y2);
    draw_dda_line(x2, y2, x1, y2);
    draw_dda_line(x1, y2, x1, y1);
}

pub fn mesh(v1: Vec<f32>, v2: Vec<f32>, space: f32)
{
    let l1 = v1.len();
    let l2 = v2.len();

    for i in 0..l1
    {
        for j in 0..l2
        {
            let x1 = v1[i] * space;
            let y1 = v2[j] * space;

            let x2: f32 = x1 + space;
            let y2: f32 = y1 + space;

            let mut x: f32 = x1;
            let mut y: f32 = y1;

            while x < x2
            {
                draw_pixel(x as i32, y1 as i32);
                x+=1.0;
            }
            while y < y2
            {
                draw_pixel(x1 as i32, y as i32);
                y+=1.0;
            }
        }
    }
}

pub fn draw_diamond(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32)
{
    let vertices: [f32; 9] = [
        // Positions
        x1, y1, 0.0,
        x2, y2, 0.0,
        x3, y3, 0.0,
    ];
    let indices: [u32; 3] = [
        0, 1, 2,
    ];

    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    crate::algorithms::raw_attributes::set_vao_vbo(&vao, &vbo, &vertices, 3);
    let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.buffer_data(&indices);

    unsafe {
        gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, ptr::null());
    }
}

#[allow(unused)]
pub fn draw_ellipse(xc: f32, yc: f32, rx: f32, ry: f32) {
    let mut x = 0.0;
    let mut y = ry;

    let mut d1 = (ry.powi(2) - rx.powi(2) * ry + 0.25 * rx.powi(2)) as f32;
    let mut dx = 2.0 * ry.powi(2) * x;
    let mut dy = 2.0 * rx.powi(2) * y;

    while dx < dy {
        draw_pixel(xc as i32 + x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 + x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 - y as i32);

        if d1 < 0.0 {
            x += 1.0;
            dx += 2.0 * ry.powi(2);
            d1 += dx + ry.powi(2);
        } else {
            x += 1.0;
            y -= 1.0;
            dx += 2.0 * ry.powi(2);
            dy -= 2.0 * rx.powi(2);
            d1 += dx - dy + ry.powi(2);
        }
    }

    let mut d2 = ((ry.powi(2) * (x + 0.5).powi(2)) + (rx.powi(2) * (y - 1.0).powi(2)) - rx.powi(2) * ry.powi(2)) as f32;

    while y >= 0.0 {
        draw_pixel(xc as i32 + x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 + x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 - y as i32);

        if d2 > 0.0 {
            y -= 1.0;
            dy -= 2.0 * rx.powi(2);
            d2 += rx.powi(2) - dy;
        } else {
            y -= 1.0;
            x += 1.0;
            dx += 2.0 * ry.powi(2);
            dy -= 2.0 * rx.powi(2);
            d2 += dx - dy + rx.powi(2);
        }

    }
}

#[allow(unused)]
pub fn draw_ellipse_color(xc: f32, yc: f32, rx: f32, ry: f32, color: &Color)
{
    let mut x = 0.0;
    let mut y = ry;

    let mut d1 = (ry.powi(2) - rx.powi(2) * ry + 0.25 * rx.powi(2)) as f32;
    let mut dx = 2.0 * ry.powi(2) * x;
    let mut dy = 2.0 * rx.powi(2) * y;

    while dx < dy {
        draw_pixel_color(xc as i32 + x as i32, yc as i32 + y as i32, color);
        draw_pixel_color(xc as i32 - x as i32, yc as i32 + y as i32, color);
        draw_pixel_color(xc as i32 + x as i32, yc as i32 - y as i32, color);
        draw_pixel_color(xc as i32 - x as i32, yc as i32 - y as i32, color);

        if d1 < 0.0 {
            x += 1.0;
            dx += 2.0 * ry.powi(2);
            d1 += dx + ry.powi(2);
        } else {
            x += 1.0;
            y -= 1.0;
            dx += 2.0 * ry.powi(2);
            dy -= 2.0 * rx.powi(2);
            d1 += dx - dy + ry.powi(2);
        }
    }

    let mut d2 = ((ry.powi(2) * (x + 0.5).powi(2)) + (rx.powi(2) * (y - 1.0).powi(2)) - rx.powi(2) * ry.powi(2)) as f32;

    while y >= 0.0 {
        draw_pixel_color(xc as i32 + x as i32, yc as i32 + y as i32, color);
        draw_pixel_color(xc as i32 - x as i32, yc as i32 + y as i32, color);
        draw_pixel_color(xc as i32 + x as i32, yc as i32 - y as i32, color);
        draw_pixel_color(xc as i32 - x as i32, yc as i32 - y as i32, color);

        if d2 > 0.0 {
            y -= 1.0;
            dy -= 2.0 * rx.powi(2);
            d2 += rx.powi(2) - dy;
        } else {
            y -= 1.0;
            x += 1.0;
            dx += 2.0 * ry.powi(2);
            dy -= 2.0 * rx.powi(2);
            d2 += dx - dy + rx.powi(2);
        }

    }
}
