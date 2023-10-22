use crate::graphics::wrapper::{Vao, Buffer, Vertex};
use crate::graphics::color::Color;
use std::ptr;

use super::lines::{bresenham_line, draw_dda_line};
use super::pixel::draw_pixel_color;

pub fn fill_rectangle(x1: f32, y1: f32, x2: f32, y2: f32)
{
    let cordenates: [f32; 12] = [
        x1, y1, 0.0, x2, y1, 0.0, x2, y2, 0.0, x1, y2, 0.0,
    ];
    let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

    let vao = Vao::new();
    vao.bind();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.buffer_data(&cordenates);

    let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.buffer_data(&indices);

    let position_attrib = Vertex::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * std::mem::size_of::<f32>() as i32,
        ptr::null(),
    );
    position_attrib.enable();

    unsafe {
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()); // Agregar fondo blanco
    }
}

pub fn fill_rectangle_inundation(x1: f32, y1: f32, x2: f32, y2: f32, color: &Color)
{
    let mut x: f32 = x1;
    let mut y: f32 = y1;

    while x <= x2
    {
        while y <= y2
        {
            draw_pixel_color(x as i32, y as i32, color);
            y += 1.0;
        }
        y = y1;
        x += 1.0;
    }
}

// TODO: There is a bug in this function, it will draw and fill the circle but at
//       compile/start time it will have a weird behavior (like screen tearing), fix that.
pub fn fill_circle_scanline(xc: f32, yc: f32, r: f32)
{
    let mut x = 0.0;
    let mut y = r;

    let mut d = 1.0 - r;

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

pub fn fill_circle_inundation(xc: f32, yc: f32, radious: f32, color: &Color)
{
    let mut x: f32 = 0.0;
    let mut y: f32 = radious;
    let mut d: f32 = 1.25 - radious;

    while x <= y
    {
        let mut i: f32 = xc - x;
        while i <= xc + x
        {
            draw_pixel_color(i as i32, (yc + y) as i32, color);
            draw_pixel_color(i as i32, (yc - y) as i32, color);
            i += 1.0;
        }
        i = xc - y;
        while i <= xc + y
        {
            draw_pixel_color(i as i32, (yc + x) as i32, color);
            draw_pixel_color(i as i32, (yc - x) as i32, color);
            i += 1.0;
        }

        if d < 0.0
        {
            d += 2.0 * x + 3.0;
        }
        else
        {
            d += 2.0 * (x - y) + 5.0;
            y -= 1.0;
        }

        x += 1.0;
    }
}

pub fn fill_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32)
{
    let vertices: [f32; 9] = [
        // Positions
        x1, y1, 0.0,
        x2, y1, 0.0,
        x3, y2, 0.0,
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

pub fn fill_triangle_inundation(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: &Color)
{
    let width = 800;
    let height = 600;

    let xpoints: [f32; 3] = [x1, x2, x3];
    let ypoints: [f32; 3] = [y1, y2, y3];

    for y in 0..height
    {
        for x in 0..width
        {
            if point_in_triangle(x as f32, y as f32, &xpoints, &ypoints)
            {
                draw_pixel_color(x, y, color);
            }
        }
    }
}

fn point_in_triangle(x: f32, y: f32, xpoints: &[f32], ypoints: &[f32]) -> bool
{
    let mut b0: bool = false;
    let mut b1: bool = false;
    let mut b2: bool = false;

    let x0 = xpoints[0];
    let y0 = ypoints[0];
    let x1 = xpoints[1];
    let y1 = ypoints[1];
    let x2 = xpoints[2];
    let y2 = ypoints[2];

    let d0 = (x - x2) * (y0 - y2) - (x0 - x2) * (y - y2);
    let d1 = (x - x0) * (y1 - y0) - (x1 - x0) * (y - y0);
    let d2 = (x - x1) * (y2 - y1) - (x2 - x1) * (y - y1);

    if d0 >= 0.0
    {
        b0 = true;
    }
    if d1 >= 0.0
    {
        b1 = true;
    }
    if d2 >= 0.0
    {
        b2 = true;
    }

    b0 == b1 && b1 == b2
}

pub fn fill_ellipse(xc: f32, yc: f32, rx: f32, ry: f32, color: &Color)
{
    let rx1 = rx.powi(2);
    let ry1 = ry.powi(2);

    let mut y = yc - ry;

    while y <= yc + ry
    {
        let dy = yc - y;
        let dx = (rx1 * (1.0 - (dy.powi(2) / ry1))).sqrt();
        bresenham_line(xc - dx, y, xc + dx, y, color);
        y += 1.0;
    }
}
