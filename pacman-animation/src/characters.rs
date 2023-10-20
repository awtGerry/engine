use engine::algorithms::fill::{fill_circle_inundation, fill_triangle_inundation};
use engine::graphics::color::Color;

const RADIUS: f32 = 10.0;

pub fn draw_pacman(x: f32, y: f32, direction: u8)
{
    fill_circle_inundation(x, y, RADIUS, &Color::new(1.0, 1.0, 0.0));
    match direction
    {
        // FACE RIGHT
        0 => fill_triangle_inundation(x+RADIUS, y+RADIUS, x+RADIUS, y-RADIUS, x, y, &Color::new(0.0, 0.0, 0.0)),
        // FACE UP
        1 => fill_triangle_inundation(x-RADIUS, y+RADIUS, x+RADIUS, y+RADIUS, x, y, &Color::new(0.0, 0.0, 0.0)),
        // FACE LEFT
        2 => fill_triangle_inundation((x-RADIUS)-0.1, y+RADIUS, (x-RADIUS)-0.1, y-RADIUS, x, y, &Color::new(0.0, 0.0, 0.0)),
        // FACE DOWN
        3 => fill_triangle_inundation(x-RADIUS, (y-RADIUS)-0.1, x+RADIUS, (y-RADIUS)-0.1, x, y, &Color::new(0.0, 0.0, 0.0)),
        _ => (),
    }
}

#[allow(unused)]
pub fn draw_red_ghost(x: f32, y: f32)
{
    fill_circle_inundation(x, y, RADIUS, &Color::new(1.0, 0.0, 0.0));
}

#[allow(unused)]
pub fn draw_pink_ghost(x: f32, y: f32)
{
    fill_circle_inundation(x, y, RADIUS, &Color::new(1.0, 0.0, 1.0));
}

#[allow(unused)]
pub fn draw_green_ghost(x: f32, y: f32)
{
    fill_circle_inundation(x, y, RADIUS, &Color::new(0.0, 1.0, 0.0));
}

#[allow(unused)]
pub fn draw_orange_ghost(x: f32, y: f32)
{
    fill_circle_inundation(x, y, RADIUS, &Color::new(1.0, 0.5, 0.0));
}
