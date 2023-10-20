use engine::graphics::window::{WIDTH, HEIGHT, Window};

mod characters;
mod movements;

fn main()
{
    let mut window = Window::new(WIDTH, HEIGHT, "Pacman animation");
    window.init();

    let mut x = (WIDTH/2) as f32;
    let mut y = (HEIGHT/2) as f32;

    let mut x2 = 1280.0;
    let mut y2 = 720.0;

    while !window.should_close()
    {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            characters::draw_pacman(x, y);
            characters::draw_red_ghost(x2, y2);
            movements::move_character(&mut x, &mut y, 1.0, 0.0);
        }
        window.update();
    }
}
