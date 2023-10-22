/* Transformations
 * 
*/

use cgmath::{Matrix4, vec3, Transform, Point3};

use crate::graphics::color::Color;

pub fn translate(x: f32, y: f32, tx: f32, ty: f32) -> (f32, f32)
{
    let v1: [[f32; 3]; 3] = [
        [1.0, 0.0, tx],
        [0.0, 1.0, ty],
        [0.0, 0.0, 1.0]
    ];

    let mut v2: [f32; 3] = [0.0; 3];

    for i in 0..3
    {
        v2[i] = v1[i][0] * x + v1[i][1] * y + v1[i][2];
    }

    let x = &v2[0];
    let y = &v2[1];
    (*x, *y)
}

/* pub fn scale(x: f32, y: f32, increment: f32)
{
    let mut translation_matrix: Matrix4<f32> = Matrix4::from_translation(vec3(0.0, 0.0, 0.0));
    translation_matrix = translation_matrix * Matrix4::from_scale(increment);

    let v1 = translation_matrix.transform_point(Point3::new(x1, y1, 0.0));
    let v2 = translation_matrix.transform_point(Point3::new(x2, y2, 0.0));
    transformation_rectangle(x1, x2, &[v1, v2], &Color::new(1.0, 0.0, 0.0));
} */
