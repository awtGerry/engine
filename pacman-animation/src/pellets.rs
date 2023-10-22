/* Pellets are the little dots that Pacman eats to gain points. They are
 * represented by a single character on the screen. */

use cgmath::{Matrix4, vec3, Transform, Point3};
use engine::graphics::color::Color;
use engine::algorithms::fill::fill_circle_inundation;

/* big_pellet will perform scalation */
pub fn big_pellet(x1: f32, y1: f32, x2: f32, y2: f32, increment: f32)
{
    let mut translation_matrix: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, 0.0));
    translation_matrix = translation_matrix * Matrix4::from_scale(increment);

    let v1 = translation_matrix.transform_point(Point3::new(x1, y1, 0.0));
    let v2 = translation_matrix.transform_point(Point3::new(x2, y2, 0.0));
    draw_big_pellet(x1, y2, &[v1, v2], &Color::new(1.0, 1.0, 1.0));
}

fn draw_big_pellet(x1: f32, y1: f32, vertices: &[Point3<f32>], color: &Color)
{
    let radious = (vertices[1].x - vertices[0].x) / 2.0;
    fill_circle_inundation(x1, y1, radious, color);
}
