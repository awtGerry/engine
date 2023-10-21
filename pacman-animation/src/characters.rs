use engine::algorithms::fill::{fill_circle_inundation, fill_triangle_inundation, fill_rectangle_inundation};
use engine::graphics::color::Color;

pub struct Ghost
{
    x: f32,
    y: f32,
    color: Color,
}

impl Ghost
{
    pub fn new(x: f32, y: f32, color: Color) -> Ghost
    {
        Ghost
        {
            x,
            y,
            color,
        }
    }
    pub fn draw(&self)
    {
        fill_rectangle_inundation(self.x-7.0, self.y-7.0, self.x+7.0, self.y+7.0, &self.color);
        fill_circle_inundation(self.x, self.y+5.0, 7.0, &self.color);

        // eyes
        fill_circle_inundation(self.x-3.0, self.y+5.0, 2.0, &Color::new(1.0, 1.0, 1.0));
        fill_circle_inundation(self.x+3.0, self.y+5.0, 2.0, &Color::new(1.0, 1.0, 1.0));
        fill_rectangle_inundation(self.x-3.0, self.y+4.5, self.x-1.0, self.y+6.5, &Color::new(0.0, 0.0, 1.0));
        fill_rectangle_inundation(self.x+3.0, self.y+4.5, self.x+5.0, self.y+6.5, &Color::new(0.0, 0.0, 1.0));

    }

    pub fn update(&mut self, x: f32, y: f32)
    {
        self.x = x;
        self.y = y;
    }
}

const RADIUS: f32 = 10.0;

pub fn draw_pacman(x: f32, y: f32, direction: u8, radius: f32)
{
    fill_circle_inundation(x, y, 10.0, &Color::new(1.0, 1.0, 0.0));
    match direction
    {
        // FACE RIGHT
        0 => fill_triangle_inundation(x+RADIUS, y+radius, x+RADIUS, y-radius, x, y, &Color::new(0.0, 0.0, 0.0)),
        // FACE UP
        1 => fill_triangle_inundation(x-radius, y+RADIUS, x+radius, y+RADIUS, x, y, &Color::new(0.0, 0.0, 0.0)),
        // FACE LEFT
        2 => fill_triangle_inundation((x-RADIUS)-0.1, y+RADIUS, (x-RADIUS)-0.1, y-RADIUS, x, y, &Color::new(0.0, 0.0, 0.0)),
        // FACE DOWN
        3 => fill_triangle_inundation(x-radius, (y-RADIUS)-0.1, x+radius, (y-RADIUS)-0.1, x, y, &Color::new(0.0, 0.0, 0.0)),
        _ => (),
    }
}

// #[allow(unused)]
// pub fn draw_red_ghost(x: f32, y: f32)
// {
//     // fill_circle_inundation(x, y, RADIUS, &Color::new(1.0, 0.0, 0.0));
//
//     fill_rectangle_inundation(x-7.0, y-7.0, x+7.0, y+7.0, &Color::new(1.0, 0.0, 0.0));
//     fill_circle_inundation(x, y+5.0, 7.0, &Color::new(1.0, 0.0, 0.0));
//     fill_triangle_inundation(x-7.0, y-7.0, x-5.0, y-7.0, x-7.0, y-10.0, &Color::new(1.0, 0.0, 0.0));
//
//     // eyes
//     fill_circle_inundation(x-3.0, y+5.0, 2.0, &Color::new(1.0, 1.0, 1.0));
//     fill_circle_inundation(x+3.0, y+5.0, 2.0, &Color::new(1.0, 1.0, 1.0));
//     fill_rectangle_inundation(x-3.0, y+4.5, x-1.0, y+6.5, &Color::new(0.0, 0.0, 1.0));
//     fill_rectangle_inundation(x+3.0, y+4.5, x+5.0, y+6.5, &Color::new(0.0, 0.0, 1.0));
// }
