use engine::graphics::window::{WIDTH, HEIGHT, Window};
use engine::graphics::color::Color;
use engine::algorithms::transformations::translate;
// use engine::algorithms::curves::draw_sun;

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

    let mut timer: f32 = 0.0;
    let mut radius: f32 = 10.0;

    let mut red_ghost = characters::Ghost::new((WIDTH/2.0)-50.0, HEIGHT/2.0, Color::new(1.0, 0.0, 0.0));
    let pink_ghost = characters::Ghost::new((WIDTH/2.0)-20.0, HEIGHT/2.0, Color::new(1.0, 0.68, 0.88));
    let blue_ghost = characters::Ghost::new((WIDTH/2.0)+20.0, HEIGHT/2.0, Color::new(0.0, 0.9, 0.8));
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
                radius = 10.0;
            }
            red_ghost.draw();
            pink_ghost.draw();
            blue_ghost.draw();
            orange_ghost.draw();
            walls::draw_walls();
            timer += 0.1;
            radius -= 1.0;
            println!("{}", timer);
        }
        window.update();
    }
}

fn pacman_animations(x: &mut f32, y: &mut f32, radius: f32, timer: &mut f32)
{
    if *timer < 46.99 {
        if *x < 465.0 {
            (*x,*y) = translate(*x, *y, 1.0, 0.0);
            characters::draw_pacman(*x, *y, 0, radius);
        }
        if *x >= 465.0 && *y < 305.0 {
            (*x,*y) = translate(*x, *y, 0.0, 1.0);
            characters::draw_pacman(*x, *y, 1, radius);
        }
        if *y >= 305.0 {
            (*x,*y) = translate(*x, *y, 1.0, 0.0);
            characters::draw_pacman(*x, *y, 0, radius);
        }
    }
    else if *timer >= 47.0 || *timer < 120.0 {
        if *x < 132.0 {
            (*x,*y) = translate(*x, *y, 1.0, 0.0);
            characters::draw_pacman(*x, *y, 0, radius);
        }
        if *x >= 132.0 {
            (*x,*y) = translate(*x, *y, 0.0, -1.0);
            characters::draw_pacman(*x, *y, 3, radius);
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
        (*red_x, *red_y) = translate(*red_x, *red_y, 1.25, 1.24);
        red_ghost.update(*red_x, *red_y);
    }
    if *timer >= 5.0 && *timer < 14.4 {
        (*red_x, *red_y) = translate(*red_x, *red_y, 1.0, 0.0);
        red_ghost.update(*red_x, *red_y);
    }
}
