use crate::graphics::color::Color;
use crate::graphics::wrapper::{Vao, Buffer};

use crate::algorithms::pixel::draw_pixel;
use crate::algorithms::raw_attributes::set_vao_vbo;

use super::lines::{draw_dda_line, bresenham_line, draw_dda_line_color};
use super::pixel::draw_pixel_color;

pub fn draw_infinity(x: f32, y: f32, r: f32)
{
    let mut t: f32 = 0.0;
    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);

    set_vao_vbo(&vao, &vbo, &[x, y], 4);

    while t <= 2.0 * std::f32::consts::PI
    {
        let _x = (r * t.cos()) + 200.0;
        let _y = (r * (2.0*t).sin())/2.0 + 100.0;

        draw_pixel(_x as i32, _y as i32);
        t+=0.01;
    }
}

pub fn draw_curve(x: f32, y: f32)
{
    let mut t: f32 = 0.0;

    while t < std::f32::consts::PI
    {
        let _x = (t * 40.0) + x;
        let _y = ((t.sin())/2.0 * 400.0) + y;

        draw_pixel(_x as i32, _y as i32);
        t+=0.01;
    }
}

pub fn draw_curve_8_points(x: f32, y: f32)
{
    let mut t: f32 = 0.0;

    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    set_vao_vbo(&vao, &vbo, &[x, y], 4);

    while t < std::f32::consts::PI
    {
        let _x = (t * 100.0) + 300.0;
        let _y = ((t.sin())/2.0 * 400.0) + 300.0;

        draw_pixel(_x as i32, _y as i32);
        t+=0.01;
    }
}

pub fn draw_flower(x: f32, y: f32)
{
    let mut t: f32 = 0.0;

    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    set_vao_vbo(&vao, &vbo, &[x, y], 4);

    while t < 360.0
    {
        let r = std::f32::consts::PI * t / 180.0;
        let _x = (r.cos() + (0.5 * (r*7.0).cos()) + (0.33 * (r*17.0).sin())) * 100.0;
        let _y = (r.sin() + (0.5 * (r*7.0).sin()) + (0.33 * (r*17.0).cos())) * 100.0;

        draw_pixel(_x as i32 + 300, _y as i32 + 300);
        t+=0.01;
    }
}

pub fn draw_humito(x: f32, y: f32)
{
    let mut y = y;
    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    set_vao_vbo(&vao, &vbo, &[x, y], 4);

    while y <= 2.0 * std::f32::consts::PI
    {
        let _x = y * (4.0 * y).cos();
        let pixel_x = (_x * 10.0) + 400.0;
        let pixel_y = (y * 10.0) + 300.0;

        draw_pixel(pixel_x as i32, pixel_y as i32);
        y+=0.01;
    }
}

pub fn draw_sun(xc: f32, yc: f32, points: u32, size: f32, color: &Color)
{
    let step: f32 = 14.0 * std::f32::consts::PI / points as f32;
    let mut x = 0.0;
    let mut y = 0.0;
    let iter = points;

    let mut is_first: bool = true;

    for i in 0..iter
    {
        let t: f32 = step * i as f32;
        let _x = 17.0 * t.cos() + 7.0 * ((17.0 / 7.0) * t).cos();
        let _y = 17.0 * t.sin() - 7.0 * ((17.0 / 7.0) * t).sin();

        let pixel_x = (_x * size) + xc;
        let pixel_y = (_y * size) + yc;

        draw_pixel_color(pixel_x as i32, pixel_y as i32, color);

        if !is_first
        {
            draw_dda_line_color(x, y, pixel_x, pixel_y, color);
        }
        else
        {
            is_first = false;
        }

        x = pixel_x;
        y = pixel_y;
    }
}
