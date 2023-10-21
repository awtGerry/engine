use engine::graphics::window::{WIDTH, HEIGHT, Window};
use engine::graphics::color::Color;
use engine::algorithms::transformations::translate;
// use engine::algorithms::curves::{draw_sun, draw_flower};

mod characters;
mod walls;

fn main()
{
    let mut window = Window::new(WIDTH, HEIGHT, "Pacman");
    window.init();

    let mut x: f32 = (WIDTH/2.0)+7.0;
    let mut y: f32 = 130.0;

    let mut red_x: f32 = (WIDTH/2.0)-50.0;
    let mut red_y: f32 = HEIGHT/2.0;

    let mut blue_x: f32 = (WIDTH/2.0)+20.0;
    let mut blue_y: f32 = HEIGHT/2.0;

    let mut timer: f32 = 0.0;
    let mut radius: f32 = 20.0;

    let mut red_ghost = characters::Ghost::new((WIDTH/2.0)-50.0, HEIGHT/2.0, Color::new(1.0, 0.0, 0.0));
    let pink_ghost = characters::Ghost::new((WIDTH/2.0)-20.0, HEIGHT/2.0, Color::new(1.0, 0.68, 0.88));
    let mut blue_ghost = characters::Ghost::new((WIDTH/2.0)+20.0, HEIGHT/2.0, Color::new(0.0, 0.9, 0.8));
    let orange_ghost = characters::Ghost::new((WIDTH/2.0)+50.0, HEIGHT/2.0, Color::new(1.0, 0.6, 0.0));

    while !window.should_close()
    {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            pacman_animations(&mut x, &mut y, radius, &mut timer);
            handle_red_animation(&mut red_x, &mut red_y, &mut timer, &mut red_ghost);
            if x >= 600.0 {
                x = 0.0;
            }
            if radius < 5.0 {
                radius = 20.0;
            }
            if timer >= 25.0 {
                handle_blue_animation(&mut blue_x, &mut blue_y, &mut timer, &mut blue_ghost);
            }
            red_ghost.draw();
            pink_ghost.draw();
            blue_ghost.draw();
            orange_ghost.draw();
            walls::draw_walls();
            timer += 0.1;
            radius -= 2.5;
            println!("{}", timer);
        }
        window.update();
    }
}

fn pacman_animations(x: &mut f32, y: &mut f32, radius: f32, timer: &mut f32)
{
    println!("Pacman: ({},{})", x, y);
    if *timer < 20.9 {
        if *x < 465.0 {
            (*x,*y) = translate(*x, *y, 1.0, 0.0);
            characters::draw_pacman(*x, *y, 0, radius);
        }
        if *x >= 465.0 && *y < 180.0 {
            (*x,*y) = translate(*x, *y, 0.0, 1.0);
            characters::draw_pacman(*x, *y, 1, radius);
        }
    }
    if *timer < 40.99 && *timer >= 20.0 {
        if *y >= 180.0 && *x <= 573.0 {
            (*x,*y) = translate(*x, *y, 1.0, 0.0);
            characters::draw_pacman(*x, *y, 0, radius);
        }
        if *x == 573.0 && *y <= 190.0 && *y > 133.0 {
            (*x,*y) = translate(*x, *y, 0.0, -1.0);
            characters::draw_pacman(*x, *y, 3, radius);
        }
        if *x >= 523.0 && *y == 133.0 {
            (*x,*y) = translate(*x, *y, -1.0, 0.0);
            characters::draw_pacman(*x, *y, 2, radius);
        }
    }

    // HANDLE IMPACT WITH GHOST
    // if *x == 465.0 {
    //     draw_sun(*x, *y, 100, 1.0, &Color::new(1.0, 1.0, 0.0));
    // }
}

fn handle_red_animation(red_x: &mut f32, red_y: &mut f32, timer: &mut f32, red_ghost: &mut characters::Ghost)
{
    // Get red ghost out of the box
    if *timer < 5.0 {
        (*red_x, *red_y) = translate(*red_x, *red_y, 1.25, 1.3);
        red_ghost.update(*red_x, *red_y);
    }
    else {
        if *red_x <= 405.0 { // Get to first corner
            (*red_x, *red_y) = translate(*red_x, *red_y, 1.0, 0.0);
            red_ghost.update(*red_x, *red_y);
        }
        if *red_x > 404.0 && *red_y >= 295.0 { // Get to second corner
            (*red_x, *red_y) = translate(*red_x, *red_y, 0.0, -1.0);
            red_ghost.update(*red_x, *red_y);
        }
        if *red_y <= 295.0 && *red_x <= 465.0 { // Get to pacman
            (*red_x, *red_y) = translate(*red_x, *red_y, 1.0, 0.0);
            red_ghost.update(*red_x, *red_y);
        }
        if *red_x >= 465.0 {
            (*red_x, *red_y) = translate(*red_x, *red_y, 0.0, -1.0);
            red_ghost.update(*red_x, *red_y);
            /* HANDLE PACMAN HIT */
        }
    }
}

fn handle_blue_animation(blue_x: &mut f32, blue_y: &mut f32, timer: &mut f32, blue_ghost: &mut characters::Ghost)
{
    if *timer < 40.0 {
        if *blue_x >= 300.0 {
            (*blue_x, *blue_y) = translate(*blue_x, *blue_y, -1.0, 1.0);
            blue_ghost.update(*blue_x, *blue_y);
        }
        if *blue_y <= 340.0 {
            (*blue_x, *blue_y) = translate(*blue_x, *blue_y, 0.0, 1.0);
            blue_ghost.update(*blue_x, *blue_y);
        }
    }
}
