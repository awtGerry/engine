use engine::graphics::color::Color;
use engine::graphics::window::{WIDTH, HEIGHT, Window};
use engine::algorithms::transformations::translate;

mod characters;
mod walls;

fn main()
{
    let mut window = Window::new(WIDTH, HEIGHT, "Pacman");
    window.init();

    let mut x: f32 = (WIDTH/2.0)+7.0;
    let mut y: f32 = 130.0;

    let mut timer: f32 = 0.0;
    let mut radius: f32 = 10.0;

    let mut red_ghost = characters::Ghost::new((WIDTH/2.0)-50.0, HEIGHT/2.0, Color::new(1.0, 0.0, 0.0));
    let mut pink_ghost = characters::Ghost::new((WIDTH/2.0)-20.0, HEIGHT/2.0, Color::new(1.0, 0.0, 1.0));
    let mut green_ghost = characters::Ghost::new((WIDTH/2.0)+20.0, HEIGHT/2.0, Color::new(0.0, 1.0, 0.0));
    let mut orange_ghost = characters::Ghost::new((WIDTH/2.0)+50.0, HEIGHT/2.0, Color::new(1.0, 0.5, 0.0));

    while !window.should_close()
    {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            if timer < 46.99 {
                first_animation(&mut x, &mut y, radius);
            }
            else {
                println!("Second transition");
                second_transition(&mut x, &mut y, radius);
            }
            if x >= 600.0 {
                x = 0.0;
            }
            if radius < 5.0 {
                radius = 10.0;
            }
            red_ghost.draw();
            pink_ghost.draw();
            green_ghost.draw();
            orange_ghost.draw();
            // characters::draw_pink_ghost((WIDTH/2.0)-20.0, HEIGHT/2.0);
            // characters::draw_green_ghost((WIDTH/2.0)+20.0, HEIGHT/2.0);
            // characters::draw_orange_ghost((WIDTH/2.0)+50.0, HEIGHT/2.0);
            walls::draw_walls();
            timer += 0.1;
            radius -= 1.0;
            println!("{}", timer);
        }
        window.update();
    }
}

fn first_animation(x: &mut f32, y: &mut f32, radius: f32)
{
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

fn second_transition(x: &mut f32, y: &mut f32, radius: f32)
{
    if *x < 132.0 {
        (*x,*y) = translate(*x, *y, 1.0, 0.0);
        characters::draw_pacman(*x, *y, 0, radius);
    }
    if *x >= 132.0 {
        (*x,*y) = translate(*x, *y, 0.0, -1.0);
        characters::draw_pacman(*x, *y, 3, radius);
    }
}
