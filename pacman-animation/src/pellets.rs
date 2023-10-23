/* Pellets are the little dots that Pacman eats to gain points. They are
 * represented by a single character on the screen. */

use cgmath::{Matrix4, vec3, Transform, Point3};
use engine::graphics::color::Color;
use engine::algorithms::fill::fill_circle_inundation;

pub struct Pellet {
    pub x: f32,
    pub y: f32,
    pub eaten: bool,
}

pub fn draw_small_pellet(pacman_x: f32, pacman_y: f32, pellets: &mut Vec<Pellet>)
{
    // Draw the pellets that have not been eaten
    for pellet in pellets.iter_mut() {
        if !pellet.eaten {
            if pellet.x > 140.0 && pellet.x < 460.0 && pellet.y > 200.0 && pellet.y < 420.0 {
                continue;
            }

            // Don't draw it if it's outside (left)
            if pellet.x >= 0.0 && pellet.x < 110.0 && pellet.y > 190.0 && pellet.y < 430.0 {
                continue;
            }

            // Don't draw it if it's outside (right)
            if pellet.x > 490.0 && pellet.x <= 600.0 && pellet.y > 190.0 && pellet.y < 430.0 {
                continue;
            }

            // Clean up so it doesn't look like the pellets are inside the walls
            if ((pellet.x >= 50.0 && pellet.x <= 110.0) && pellet.y > 158.0 && pellet.y < 170.0) ||
                (pellet.x >= 150.0 && pellet.x <= 250.0 && pellet.y > 158.0 && pellet.y < 170.0) ||
                (pellet.x >= 150.0 && pellet.x <= 180.0 && pellet.y > 70.0 && pellet.y < 130.0) ||
                (pellet.x >= 340.0 && pellet.x <= 440.0 && pellet.y > 158.0 && pellet.y < 170.0)
            {
                continue;
            }

            fill_circle_inundation(pellet.x, pellet.y, 2.0, &Color::new(1.0, 1.0, 1.0));
        }
    }

    // Check if Pacman passed through the pellets
    for pellet in pellets.iter_mut() {
        if !pellet.eaten && (pacman_x - pellet.x).abs() < 10.0 && (pacman_y - pellet.y).abs() < 10.0 {
            // Mark the pellet as "eaten"
            pellet.eaten = true;
        }
    }
}

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
