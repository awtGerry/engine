#[allow(unused_imports)]
use engine::graphics::window::{WIDTH, HEIGHT};
use engine::graphics::color::Color;
use engine::algorithms::fill::fill_rectangle_inundation;

pub fn draw_walls()
{

    let wall_color: Color = Color::new(0.0, 0.25, 0.8);
    fill_rectangle_inundation(0.0, 0.0, WIDTH, 10.0, &wall_color); // Bottom wall
    fill_rectangle_inundation(0.0, HEIGHT-10.0, WIDTH, HEIGHT, &wall_color); // Top wall

    fill_rectangle_inundation(0.0, 400.0, 10.0, HEIGHT, &wall_color); // Left top wall
    fill_rectangle_inundation(0.0, 0.0, 10.0, 200.0, &wall_color); // Left bottom wall
    fill_rectangle_inundation(WIDTH-10.0, 400.0, WIDTH, HEIGHT, &wall_color); // Right top wall
    fill_rectangle_inundation(WIDTH-10.0, 0.0, WIDTH, 200.0, &wall_color); // Right bottom wall

    fill_rectangle_inundation(0.0, 200.0, 100.0, 210.0, &wall_color); // Left to mid wall (bottom)
    fill_rectangle_inundation(100.0, 200.0, 110.0, 260.0, &wall_color); // Left going up wall (bottom)
    fill_rectangle_inundation(0.0, 260.0, 110.0, 270.0, &wall_color); // Left back to 0 wall (bottom)

    fill_rectangle_inundation(0.0, 390.0, 100.0, 400.0, &wall_color); // Left to mid wall (top)
    fill_rectangle_inundation(100.0, 340.0, 110.0, 400.0, &wall_color); // Left going down wall (top)
    fill_rectangle_inundation(0.0, 340.0, 110.0, 350.0, &wall_color); // Left back to 0 wall (top)

    fill_rectangle_inundation(WIDTH-100.0, 200.0, WIDTH, 210.0, &wall_color); // Right to mid wall (bottom)
    fill_rectangle_inundation(WIDTH-110.0, 200.0, WIDTH-100.0, 260.0, &wall_color); // Right going up wall (bottom)
    fill_rectangle_inundation(WIDTH-110.0, 260.0, WIDTH, 270.0, &wall_color); // Right back to 0 wall (bottom)

    fill_rectangle_inundation(WIDTH-100.0, 400.0, WIDTH, 410.0, &wall_color); // Right to mid wall (top)
    fill_rectangle_inundation(WIDTH-110.0, 340.0, WIDTH-100.0, 410.0, &wall_color); // Right going down wall (top)
    fill_rectangle_inundation(WIDTH-110.0, 340.0, WIDTH, 350.0, &wall_color); // Right back to 0 wall (top)

    inner_obstacles();
}

fn inner_obstacles()
{
    let wall_color: Color = Color::new(0.0, 0.25, 0.8);
    fill_rectangle_inundation(0.0, 100.0, 50.0, 110.0, &wall_color); // Left bottom obstacle
    fill_rectangle_inundation(WIDTH-50.0, 100.0, WIDTH, 110.0, &wall_color); // Right bottom obstacle

    fill_rectangle_inundation(100.0, 100.0, 110.0, 160.0, &wall_color); // Left middle obstacle
    fill_rectangle_inundation(WIDTH-110.0, 100.0, WIDTH-100.0, 160.0, &wall_color); // Right middle obstacle
}
