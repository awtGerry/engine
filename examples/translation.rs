// use engine::algorithms::fill::{fill_rectangle, fill_rectangle_inundation, fill_circle_inundation};
// use engine::algorithms::lines::bresenham_line;
// use engine::algorithms::pixel::draw_pixel_color;
// use engine::algorithms::transformations::translate;
use engine::algorithms::curves::draw_sun;
use engine::graphics::color::Color;
use engine::graphics::window::Window;
use cgmath::{Matrix4, vec3, Rad, Transform, Point3};

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 720.0;

fn main()
{
    let mut window = Window::new(WIDTH, HEIGHT, "Traslation");
    window.init();

    let x1 = 300.0;
    let y1 = 300.0;
    let x2 = 350.0;
    let y2 = 350.0;

    let center_x = (x1 + x2) / 2.0;
    let center_y = (y1 + y2) / 2.0;

    let mut increment = 0.0;

    while !window.should_close()
    {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            let rotation_matrix: Matrix4<f32> = Matrix4::from_angle_z(Rad(increment));
            let translation_matrix: Matrix4<f32> = Matrix4::from_translation(vec3(center_x, center_y, 0.0));
            let transformation_matrix = translation_matrix * rotation_matrix * translation_matrix.inverse_transform().unwrap();

            let v1 = transformation_matrix.transform_point(Point3::new(x1, y1, 0.0));
            let v2 = transformation_matrix.transform_point(Point3::new(x2, y1, 0.0));
            let v3 = transformation_matrix.transform_point(Point3::new(x2, y2, 0.0));
            let v4 = transformation_matrix.transform_point(Point3::new(x1, y2, 0.0));


            // let mut translation_matrix: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, 0.0));
            // translation_matrix = translation_matrix * Matrix4::from_scale(increment);
            // let v1 = translation_matrix.transform_point(Point3::new(x1, y1, 0.0));
            // let v2 = translation_matrix.transform_point(Point3::new(x2, y2, 0.0));

            transformation_rectangle(&[v1, v2, v3, v4], &Color::new(0.0, 1.0, 0.0));

            // if increment > 1.0 {
            //     increment = 0.0;
            // }
            increment += 0.01;
        }
        window.update();
    }

}

fn transformation_rectangle(vertices: &[Point3<f32>], color: &Color)
{
    for i in 0..4 {
        // draw_pixel_color(vertices[i].x as i32, vertices[i].y as i32, color);
        draw_sun(vertices[i].x, vertices[i].y, 100, 10.0, color);
    }
}
