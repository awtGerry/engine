use engine::algorithms::fill::fill_circle_inundation;

const RADIUS: f32 = 16.0;

pub fn draw_pacman(x: f32, y: f32)
{
    fill_circle_inundation(x, y, RADIUS, 1.0, 1.0, 0.0);
    // eye
    fill_circle_inundation(x-2.0, y+8.0, 2.0, 0.0, 0.0, 0.0);
    // make the mouth like a V form
    fill_circle_inundation(x+15.0, y, 10.0, 0.0, 0.0, 0.0);
}

pub fn draw_red_ghost(x: f32, y: f32)
{
    fill_circle_inundation(x, y, RADIUS, 1.0, 0.0, 0.0);
}

pub fn draw_pink_ghost(x: f32, y: f32)
{
    fill_circle_inundation(x, y, RADIUS, 1.0, 0.0, 1.0);
}

pub fn draw_green_ghost(x: f32, y: f32)
{
    fill_circle_inundation(x, y, RADIUS, 0.0, 1.0, 0.0);
}

pub fn draw_orange_ghost(x: f32, y: f32)
{
    fill_circle_inundation(x, y, RADIUS, 1.0, 0.5, 0.0);
}
