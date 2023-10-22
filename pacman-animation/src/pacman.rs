use engine::graphics::window::{WIDTH, HEIGHT, Window};
use engine::graphics::color::Color;
// use engine::algorithms::transformations::translate;
use rand::Rng;
// use engine::algorithms::curves::{draw_sun, draw_flower};

mod characters;
mod walls;
mod pellets;

fn main()
{
    let mut window = Window::new(WIDTH, HEIGHT, "Pacman");
    window.init();

    let x: f32 = (WIDTH/2.0)+7.0;
    let y: f32 = 130.0;

    let mut radius: f32 = 20.0;
    let mut increment: f32 = 0.0;
    // Timer (seconds) to control the ghost's movement
    let mut timer: f32 = 0.0;

    // Chose a random start direction for pacman (just left or right)
    let mut rng = rand::thread_rng();
    let mut direction: characters::Direction = characters::Direction::Left;
    if rng.gen_range(0..2) == 1 {
        direction = characters::Direction::Right;
    }
    let mut pacman = characters::Pacman::new(x, y, direction, 10.0);

    let red_ghost = characters::Ghost::new((WIDTH/2.0)-50.0, HEIGHT/2.0, Color::new(1.0, 0.0, 0.0));
    let pink_ghost = characters::Ghost::new((WIDTH/2.0)-20.0, HEIGHT/2.0, Color::new(1.0, 0.68, 0.88));
    let blue_ghost = characters::Ghost::new((WIDTH/2.0)+20.0, HEIGHT/2.0, Color::new(0.0, 0.9, 0.8));
    let orange_ghost = characters::Ghost::new((WIDTH/2.0)+50.0, HEIGHT/2.0, Color::new(1.0, 0.6, 0.0));

    while !window.should_close()
    {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            if radius < 5.0 {
                radius = 20.0;
            }
            if increment > 0.6 {
                increment = 0.0;
            }
            pellets::big_pellet(37.0, 500.0, 57.0, 520.0, increment);
            pellets::big_pellet(563.0, 500.0, 583.0, 520.0, increment);
            pellets::big_pellet(35.0, 110.0, 55.0, 130.0, increment);
            pellets::big_pellet(565.0, 110.0, 585.0, 130.0, increment);

            pacman.move_pacman();
            pacman.draw(radius);
            red_ghost.draw();
            pink_ghost.draw();
            blue_ghost.draw();
            orange_ghost.draw();
            walls::draw_walls();

            radius -= 2.5;
            increment += 0.1;
            timer += 0.01;
        }
        window.update();
    }
}
