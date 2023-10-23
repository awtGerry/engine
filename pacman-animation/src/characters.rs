use engine::algorithms::fill::{fill_circle_inundation, fill_triangle_inundation, fill_rectangle_inundation};
use engine::algorithms::transformations::translate;
use engine::algorithms::curves::draw_sun;
use engine::graphics::color::Color;
use cgmath::{Matrix4, vec3, Rad, Transform, Point3};
use rand::Rng;

#[derive(PartialEq)]
pub enum Direction
{
    Right,
    Up,
    Left,
    Down,
}

pub struct Ghost
{
    x: f32,
    y: f32,
    direction: Direction,
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
            direction: Direction::Up,
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

    #[allow(unused)]
    pub fn update(&mut self, x: f32, y: f32)
    {
        self.x = x;
        self.y = y;
    }

    #[allow(unused)]
    /* HANDLE COLOR CHANGE */
    pub fn update_color(&mut self, color: Color)
    {
        self.color = color;
    }

    #[allow(unused)]
    pub fn move_ghost(&mut self)
    {
        match self.direction
        {
            Direction::Up => (self.x, self.y) = translate(self.x, self.y, 0.0, 2.0),
            Direction::Down => (self.x, self.y) = translate(self.x, self.y, 0.0, -2.0),
            Direction::Left => (self.x, self.y) = translate(self.x, self.y, -2.0, 0.0),
            Direction::Right => (self.x, self.y) = translate(self.x, self.y, 2.0, 0.0),
        }

        if self.wall_collision(self.x, self.y)
        {
            self.change_direction();
        }
    }

    fn change_direction(&mut self)
    {
        let mut rng = rand::thread_rng();

        // Generate a random direction (excluding the current direction)
        let new_direction = loop {
            let random_direction = match rng.gen_range(0..4) {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Right,
                _ => Direction::Up, // Default to Up in case of unexpected random value
            };

            if random_direction != self.direction {
                break random_direction;
            }
        };

        self.direction = new_direction;
    }

    fn wall_collision(&mut self, x: f32,y: f32) -> bool
    {
        /* FIRST ONES ARE THE OUTER WALLS ALLWAYS */

        // HANDLE DOWN WALL COLLISION
        if y <= 26.0 || (x <= 127.0 && y == 74.0) || (x >= 230.0 && x <= 370.0 && y == 470.0)
        {
            self.y += 1.0;
            return true;
        }

        // HANDLE UP WALL COLLISION
        if y >= 574.0 || (x == 27.0 && y == 83.0) ||
            (x >= 230.0 && x <= 370.0 && y == 440.0)
        {
            self.y -= 1.0;
            return true;
        }

        // HANDLE LEFT WALL COLLISION
        if (x <= 26.0 && y != 300.0) || (x <= 126.0 && y == 130.0) {
            self.x += 1.0;
            return true;
        }

        // HANDLE RIGHT WALL COLLISION
        if (x >= 574.0 && y != 300.0) || (x >= 474.0 && y == 130.0) || (x == 154.0 && y == 75.0)
        {
            self.x -= 1.0;
            return true;
        }

        false
    }

    #[allow(unused)]
    pub fn get_x(&self) -> f32
    {
        self.x
    }

    #[allow(unused)]
    pub fn get_y(&self) -> f32
    {
        self.y
    }
}

pub struct Pacman
{
    x: f32,
    y: f32,
    direction: Direction,
    radius: f32,
}

impl Pacman
{
    pub fn new(x: f32, y: f32, direction: Direction, radius: f32) -> Pacman
    {
        Pacman
        {
            x,
            y,
            direction,
            radius,
        }
    }

    pub fn move_pacman(&mut self)
    {
        // USE TRANSLATION TO MOVE PACMAN
        match self.direction
        {
            Direction::Up => (self.x, self.y) = translate(self.x, self.y, 0.0, 1.0),
            Direction::Down => (self.x, self.y) = translate(self.x, self.y, 0.0, -1.0),
            Direction::Left => (self.x, self.y) = translate(self.x, self.y, -1.0, 0.0),
            Direction::Right => (self.x, self.y) = translate(self.x, self.y, 1.0, 0.0),
        }

        println!("x: {}, y: {}", self.x, self.y);

        if self.wall_collision(self.x, self.y)
        {
            self.change_direction();
        }

        if self.x <= 1.0 && (self.y >= 290.0 && self.y<= 310.0)
        {
            self.x = 599.0;
        }
    }

    fn change_direction(&mut self)
    {
        let mut rng = rand::thread_rng();

        // Generate a random direction (excluding the current direction)
        let new_direction = loop {
            let random_direction = match rng.gen_range(0..4) {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Right,
                _ => Direction::Up, // Default to Up in case of unexpected random value
            };

            if random_direction != self.direction {
                break random_direction;
            }
        };

        self.direction = new_direction;
    }

    pub fn user_change_direction(&mut self, direction: Direction)
    {
        self.direction = direction;
    }

    pub fn draw(&self, mounth: f32)
    {
        fill_circle_inundation(self.x, self.y, self.radius, &Color::new(1.0, 1.0, 0.0));
        match self.direction
        {
            Direction::Up => {
                fill_triangle_inundation(self.x - mounth, self.y+self.radius, self.x+mounth, self.y+self.radius, self.x, self.y, &Color::new(0.0, 0.0, 0.0));
            },
            Direction::Down => {
                fill_triangle_inundation(self.x-mounth, (self.y-self.radius)-0.1, self.x+mounth, (self.y-self.radius)-0.1, self.x, self.y, &Color::new(0.0, 0.0, 0.0));
            },
            Direction::Left => {
                fill_triangle_inundation((self.x-self.radius)-0.1, self.y+mounth, (self.x-self.radius)-0.1, self.y-mounth, self.x, self.y, &Color::new(0.0, 0.0, 0.0));
            },
            Direction::Right => {
                fill_triangle_inundation(self.x+self.radius, self.y+mounth, self.x+self.radius, self.y-mounth, self.x, self.y, &Color::new(0.0, 0.0, 0.0));
            },
        }
    }

    pub fn get_x(&self) -> f32
    {
        self.x
    }

    pub fn get_y(&self) -> f32
    {
        self.y
    }

    fn wall_collision(&mut self, x: f32,y: f32) -> bool
    {

        /* FIRST ONES ARE THE OUTER WALLS ALLWAYS */

        // HANDLE DOWN WALL COLLISION
        if y <= 26.0 || (x <= 127.0 && y == 74.0) || (x >= 230.0 && x <= 370.0 && y == 470.0)
        {
            self.y += 1.0;
            return true;
        }

        // HANDLE UP WALL COLLISION
        if y >= 574.0 || (x == 27.0 && y == 83.0) ||
            (x >= 230.0 && x <= 370.0 && y == 440.0)
        {
            self.y -= 1.0;
            return true;
        }

        // HANDLE LEFT WALL COLLISION
        if (x <= 26.0 && y != 300.0) || (x <= 126.0 && y == 130.0) {
            self.x += 1.0;
            return true;
        }

        // HANDLE RIGHT WALL COLLISION
        if (x >= 574.0 && y != 300.0) || (x >= 474.0 && y == 130.0) || (x == 154.0 && y == 75.0)
        {
            self.x -= 1.0;
            return true;
        }

        false
    }

    pub fn handle_death(&mut self, increment: f32)
    {
        let rotation_matrix: Matrix4<f32> = cgmath::Matrix4::from_angle_z(Rad(increment));
        let translation_matrix: Matrix4<f32> = cgmath::Matrix4::from_translation(vec3(self.x, self.y, 0.0));
        let transformation_matrix = translation_matrix * rotation_matrix * translation_matrix.inverse_transform().unwrap();

        let v1 = transformation_matrix.transform_point(Point3::new(self.x-7.0, self.y-7.0, 0.0));
        let v2 = transformation_matrix.transform_point(Point3::new(self.x+7.0, self.y-7.0, 0.0));
        let v3 = transformation_matrix.transform_point(Point3::new(self.x+7.0, self.y+7.0, 0.0));
        let v4 = transformation_matrix.transform_point(Point3::new(self.x-7.0, self.y+7.0, 0.0));
        self.rotate_sun(&[v1, v2, v3, v4], &Color::new(1.0, 1.0, 0.0));
    }

    fn rotate_sun(&mut self, vertices: &[Point3<f32>], color: &Color)
    {
        for i in 0..vertices.len() {
            draw_sun(vertices[i].x, vertices[i].y, 100, 2.0, color);
        }
    }
}
