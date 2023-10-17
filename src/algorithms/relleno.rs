use crate::graphics::window::{WIDTH, HEIGHT};
use crate::algorithms::pixel::draw_pixel;

#[allow(unused)]
pub fn indundacion(x: u32, y: u32)
{
    if x <= 0 || x >= WIDTH || y <= 0 || y >= HEIGHT
    {
        return;
    }

    draw_pixel(x as i32, y as i32);

    indundacion(x + 1, y);
    indundacion(x - 1, y);
    indundacion(x, y + 1);
    indundacion(x, y - 1);

}
