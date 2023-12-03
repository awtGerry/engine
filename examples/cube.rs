// 3D cube using projections
// To make 3D work, we need to use a projection matrix
// This is a matrix that will transform 3D coordinates into 2D coordinates

use engine::algorithms::cs::projection_cube;
use engine::graphics::window::Window;
use engine::graphics::color::Color;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

fn main()
{
    let mut window = Window::new(WIDTH, HEIGHT, "3D Cube");
    window.init();

    let vp: [f32; 3] = [30.0, 30.0, 500.0];
    let points = vec![
        vec![WIDTH / 4.0, HEIGHT / 4.0, 100.0],
        vec![WIDTH / 4.0, HEIGHT * 3.0 / 4.0, 100.0],
        vec![WIDTH * 3.0 / 4.0, HEIGHT / 4.0, 100.0],
        vec![WIDTH * 3.0 / 4.0, HEIGHT * 3.0 / 4.0, 100.0],
        vec![WIDTH / 4.0, HEIGHT / 4.0, 400.0],
        vec![WIDTH / 4.0, HEIGHT * 3.0 / 4.0, 400.0],
        vec![WIDTH * 3.0 / 4.0, HEIGHT / 4.0, 400.0],
        vec![WIDTH * 3.0 / 4.0, HEIGHT * 3.0 / 4.0, 400.0],
    ];
    // let points = vec![
    //     vec![100.0, 100.0, 100.0],
    //     vec![100.0, 400.0, 100.0],
    //     vec![400.0, 100.0, 100.0],
    //     vec![400.0, 400.0, 100.0],
    //     vec![100.0, 100.0, 400.0],
    //     vec![100.0, 400.0, 400.0],
    //     vec![400.0, 100.0, 400.0],
    //     vec![400.0, 400.0, 400.0],
    // ];
    let color: Color = Color::new(0.0, 0.5, 1.0);

    while !window.should_close()
    {
        unsafe
        {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            projection_cube(&points, &vp, &color);
        }
        window.update();
    }
}
