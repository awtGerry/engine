use engine::{algorithms::fill::fill_rectangle_inundation, graphics::color::Color};
#[allow(unused_imports)]
use engine::graphics::window::{WIDTH, HEIGHT};

pub struct Wall {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

pub fn walls_cords() -> Vec<Wall>
{
    let mut walls: Vec<Wall> = Vec::new();
    outside(&mut walls);
    ghost_rectangle(&mut walls);
    inner_obstacles_bottom(&mut walls);
    inner_obstacles_top(&mut walls);

    walls
}

pub fn get_walls() -> Vec<Wall>
{
    walls_cords()
}

pub fn draw_walls()
{
    let walls = get_walls();
    for wall in walls {
        fill_rectangle_inundation(wall.x1, wall.y1, wall.x2, wall.y2, &Color::new(0.0, 0.0, 1.0));
    }
}

fn ghost_rectangle(walls: &mut Vec<Wall>)
{
    walls.push(Wall { x1: 220.0, y1: 260.0, x2: 380.0, y2: 270.0 }); // Left to right wall (bottom)
    // walls.push(Wall { x1: 220.0, y1: 345.0, x2: 380.0, y2: 347.0 }); // Left to right wall (top)
    walls.push(Wall { x1: 220.0, y1: 260.0, x2: 230.0, y2: 350.0 }); // Left to right wall (left)
    walls.push(Wall { x1: 370.0, y1: 260.0, x2: 380.0, y2: 350.0 }); // Left to right wall (right)
}

fn inner_obstacles_bottom(walls: &mut Vec<Wall>)
{
    /* LEFT BOTTOM OBSTACLES */
    walls.push(Wall { x1: 0.0, y1: 100.0, x2: 50.0, y2: 110.0 }); // Left bottom obstacle
    walls.push(Wall { x1: 100.0, y1: 100.0, x2: 110.0, y2: 160.0 }); // L shape
    walls.push(Wall { x1: 60.0, y1: 150.0, x2: 110.0, y2: 160.0 }); // L shape

    walls.push(Wall { x1: 50.0, y1: 50.0, x2: 250.0, y2: 60.0 }); // Left middle obstacle
    walls.push(Wall { x1: 160.0, y1: 50.0, x2: 170.0, y2: 110.0 }); // Left middle obstacle

    walls.push(Wall { x1: 160.0, y1: 200.0, x2: 170.0, y2: 270.0 }); // Left middle obstacle

    walls.push(Wall { x1: 160.0, y1: 150.0, x2: 240.0, y2: 160.0 }); // left small wall

    /* MID BOTTOM OBSTACLES */
    walls.push(Wall { x1: 300.0, y1: 150.0, x2: 310.0, y2: 210.0 }); // T shape (top)
    walls.push(Wall { x1: 300.0, y1: 200.0, x2: 380.0, y2: 210.0 }); // T shape (top)
    walls.push(Wall { x1: 220.0, y1: 200.0, x2: 300.0, y2: 210.0 }); // T shape (top)

    walls.push(Wall { x1: 300.0, y1: 50.0, x2: 310.0, y2: 110.0 }); // T shape (bottom)
    walls.push(Wall { x1: 300.0, y1: 100.0, x2: 380.0, y2: 110.0 }); // T shape (bottom)
    walls.push(Wall { x1: 220.0, y1: 100.0, x2: 300.0, y2: 110.0 }); // T shape (bottom)

    /* RIGHT BOTTOM OBSTACLES */
    walls.push(Wall { x1: WIDTH-110.0, y1: 100.0, x2: WIDTH-100.0, y2: 160.0 }); // Right middle obstacle
    walls.push(Wall { x1: WIDTH-110.0, y1: 150.0, x2: WIDTH-60.0, y2: 160.0 }); // Right middle obstacle
    walls.push(Wall { x1: WIDTH-50.0, y1: 100.0, x2: WIDTH, y2: 110.0 }); // Right bottom obstacle

    walls.push(Wall { x1: WIDTH-250.0, y1: 50.0, x2: WIDTH-50.0, y2: 60.0 }); // Right middle obstacle
    walls.push(Wall { x1: WIDTH-170.0, y1: 50.0, x2: WIDTH-160.0, y2: 110.0 }); // Right middle obstacle

    walls.push(Wall { x1: 430.0, y1: 200.0, x2: 440.0, y2: 270.0 }); // Right middle obstacle

    walls.push(Wall { x1: 360.0, y1: 150.0, x2: 440.0, y2: 160.0 }); // right small wall
}

fn inner_obstacles_top(walls: &mut Vec<Wall>)
{
    walls.push(Wall { x1: 300.0, y1: 500.0, x2: 310.0, y2: 600.0 }); // Top T with outer wall

    walls.push(Wall { x1: 230.0, y1: 450.0, x2: 370.0, y2: 460.0 }); // Top T
    walls.push(Wall { x1: 300.0, y1: 400.0, x2: 310.0, y2: 460.0 }); // Top T

    /* LEFT */
    walls.push(Wall { x1: 160.0, y1: 340.0, x2: 170.0, y2: 460.0 }); // Mid inverted T
    walls.push(Wall { x1: 160.0, y1: 400.0, x2: 240.0, y2: 410.0 }); // Mid inverted T

    walls.push(Wall { x1: 60.0, y1: 450.0, x2: 110.0, y2: 460.0 }); // Little left wall

    walls.push(Wall { x1: 60.0, y1: 500.0, x2: 110.0, y2: 540.0 }); // Rectangle 1
    walls.push(Wall { x1: 160.0, y1: 500.0, x2: 240.0, y2: 540.0 }); // Rectangle 2

    /* RIGHT */
    walls.push(Wall { x1: 430.0, y1: 340.0, x2: 440.0, y2: 460.0 }); // Mid inverted T
    walls.push(Wall { x1: 360.0, y1: 400.0, x2: 440.0, y2: 410.0 }); // Mid inverted T

    walls.push(Wall { x1: WIDTH-100.0, y1: 450.0, x2: WIDTH-60.0, y2: 460.0 }); // Little right wall

    walls.push(Wall { x1: WIDTH-100.0, y1: 500.0, x2: WIDTH-60.0, y2: 540.0 }); // Rectangle 1
    walls.push(Wall { x1: WIDTH-240.0, y1: 500.0, x2: WIDTH-160.0, y2: 540.0 }); // Rectangle 2
}

fn outside(walls: &mut Vec<Wall>)
{
    walls.push(Wall { x1: 0.0, y1: 0.0, x2: WIDTH, y2: 10.0 }); // Bottom wall
    walls.push(Wall { x1: 0.0, y1: HEIGHT-10.0, x2: WIDTH, y2: HEIGHT }); // Top wall

    walls.push(Wall { x1: 0.0, y1: 0.0, x2: 10.0, y2: HEIGHT }); // Left top wall
    walls.push(Wall { x1: 0.0, y1: 0.0, x2: 10.0, y2: 200.0 }); // Left bottom wall
    walls.push(Wall { x1: WIDTH-10.0, y1: 0.0, x2: WIDTH, y2: HEIGHT }); // Right top wall
    walls.push(Wall { x1: WIDTH-10.0, y1: 0.0, x2: WIDTH, y2: 200.0 }); // Right bottom wall

    walls.push(Wall { x1: 0.0, y1: 200.0, x2: 100.0, y2: 210.0 }); // Left to mid wall (bottom)
    walls.push(Wall { x1: 100.0, y1: 200.0, x2: 110.0, y2: 260.0 }); // Left going up wall (bottom)
    walls.push(Wall { x1: 0.0, y1: 260.0, x2: 110.0, y2: 270.0 }); // Left back to 0 wall (bottom)

    walls.push(Wall { x1: 0.0, y1: 400.0, x2: 100.0, y2: 410.0 }); // Left to mid wall (top)
    walls.push(Wall { x1: 100.0, y1: 340.0, x2: 110.0, y2: 410.0 }); // Left going down wall (top)
    walls.push(Wall { x1: 0.0, y1: 340.0, x2: 110.0, y2: 350.0 }); // Left back to 0 wall (top)

    walls.push(Wall { x1: WIDTH-100.0, y1: 200.0, x2: WIDTH, y2: 210.0 }); // Right to mid wall (bottom)
    walls.push(Wall { x1: WIDTH-110.0, y1: 200.0, x2: WIDTH-100.0, y2: 260.0 }); // Right going up wall (bottom)
    walls.push(Wall { x1: WIDTH-110.0, y1: 260.0, x2: WIDTH, y2: 270.0 }); // Right back to 0 wall (bottom)

    walls.push(Wall { x1: WIDTH-100.0, y1: 400.0, x2: WIDTH, y2: 410.0 }); // Right to mid wall (top)
    walls.push(Wall { x1: WIDTH-110.0, y1: 340.0, x2: WIDTH-100.0, y2: 410.0 }); // Right going down wall (top)
    walls.push(Wall { x1: WIDTH-110.0, y1: 340.0, x2: WIDTH, y2: 350.0 }); // Right back to 0 wall (top)
}
