use engine::{graphics::window::{WIDTH, HEIGHT, Window}, algorithms::transformations::translate};

mod characters;
mod walls;

fn main()
{
    let mut window = Window::new(WIDTH, HEIGHT, "Pacman");
    window.init();

    let x: f32 = 120.0;
    let y: f32 = 180.0;

    // let mut timer: f32 = 0.0;

    while !window.should_close()
    {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // if timer < 100.0 {
            //     if x < 150.0 {
            //         (x,y) = translate(x, y, 1.0, 0.0);
            //         characters::draw_pacman(x, y, 0);
            //     }
            //     if x >= 150.0 {
            //         (x,y) = translate(x, y, 0.0, 1.0);
            //         characters::draw_pacman(x, y, 1);
            //     }
            // }
            characters::draw_pacman(x, y, 0);
            walls::draw_walls();
            // timer += 0.1;
            // println!("{}", timer);
        }
        window.update();
    }
}
