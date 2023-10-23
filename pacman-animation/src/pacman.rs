use engine::algorithms::curves;
use engine::algorithms::fill::fill_rectangle_inundation;
use engine::graphics::window::{WIDTH, HEIGHT, Window};
use engine::graphics::color::Color;
use glfw::{WindowEvent, Key, Action};
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

    let mut hit_ghost: bool = false;

    let mut radius: f32 = 10.0;
    let mut increment: f32 = 0.0;
    // Timer (seconds) to control the ghost's movement
    let mut timer: f32 = 0.0;

    // Chose a random start direction for pacman (just left or right)
    let mut rng = rand::thread_rng();
    let mut direction: characters::Direction = characters::Direction::Left;
    if rng.gen_range(0..2) == 1 {
        direction = characters::Direction::Right;
    }
    let mut pacman = characters::Pacman::new(x, y, direction, 12.0);

    let red_ghost = characters::Ghost::new((WIDTH/2.0)-50.0, HEIGHT/2.0, Color::new(1.0, 0.0, 0.0));
    let pink_ghost = characters::Ghost::new((WIDTH/2.0)-20.0, HEIGHT/2.0, Color::new(1.0, 0.68, 0.88));
    let blue_ghost = characters::Ghost::new((WIDTH/2.0)+20.0, HEIGHT/2.0, Color::new(0.0, 0.9, 0.8));
    let orange_ghost = characters::Ghost::new((WIDTH/2.0)+50.0, HEIGHT/2.0, Color::new(1.0, 0.6, 0.0));

    // Put the ghosts in a vector to iterate over them
    let mut ghosts: Vec<characters::Ghost> = vec![red_ghost, pink_ghost, blue_ghost, orange_ghost];
    let mut consumed_pellets = Vec::new();
    // Draw the pellets
    for i in (0..600).step_by(25) {
        for j in (0..600).step_by(27) {
            let pellet = pellets::Pellet {
                x: i as f32,
                y: j as f32,
                eaten: false,
            };
            consumed_pellets.push(pellet);
        }
    }

    while !window.should_close()
    {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if radius < 5.0 {
                radius = 10.0;
            }
            if increment > 0.6 {
                increment = 0.0;
            }

            // Move the ghosts
            ghosts[0].move_ghost();
            if timer >= 2.5 { ghosts[2].move_ghost(); }
            if timer >= 5.0 { ghosts[1].move_ghost(); }
            if timer >= 7.5 { ghosts[3].move_ghost(); }

            for ghost in &mut ghosts {
                if is_collision(&pacman, ghost) {
                    hit_ghost = true;
                    break;
                }
            }

            if hit_ghost {
                pacman.handle_death(timer);
                // walls::get_walls();
                walls::draw_walls();
            } else {
                process_events(&mut window, &mut pacman);
                pacman.move_pacman();
                pacman.draw(radius);
                ghosts[0].draw();
                ghosts[1].draw();
                ghosts[2].draw();
                ghosts[3].draw();

                pellets::draw_small_pellet(pacman.get_x(), pacman.get_y(), &mut consumed_pellets);
                pellets::big_pellet(25.0, 493.0, 45.0, 513.0, increment);
                pellets::big_pellet(575.0, 493.0, 595.0, 513.0, increment);
                pellets::big_pellet(25.0, 140.0, 45.0, 160.0, increment);
                pellets::big_pellet(575.0, 140.0, 595.0, 160.0, increment);

                // walls::get_walls();
                walls::draw_walls();
            }

            radius -= 2.5;
            increment += 0.1;
            timer += 0.01;
        }
        window.update();
    }
}

fn process_events(window: &mut Window, pacman: &mut characters::Pacman)
{
    for (_, event) in glfw::flush_messages(&window.events) {
        match event {
            WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height);
            },
            WindowEvent::Key(Key::Up, _, Action::Press, _) => {
                characters::Pacman::user_change_direction(pacman, characters::Direction::Up);
            },
            WindowEvent::Key(Key::Down, _, Action::Press, _) => {
                characters::Pacman::user_change_direction(pacman, characters::Direction::Down);
            },
            WindowEvent::Key(Key::Left, _, Action::Press, _) => {
                characters::Pacman::user_change_direction(pacman, characters::Direction::Left);
            },
            WindowEvent::Key(Key::Right, _, Action::Press, _) => {
                characters::Pacman::user_change_direction(pacman, characters::Direction::Right);
            },
            _ => {}
        }
    }
}

fn is_collision(pacman: &characters::Pacman, ghost: &characters::Ghost) -> bool
{
    let distance = ((pacman.get_x() - ghost.get_x()).powi(2) + (pacman.get_y() - ghost.get_y()).powi(2)).sqrt();
    distance < 10.0 // Adjust COLLISION_THRESHOLD as needed
}
