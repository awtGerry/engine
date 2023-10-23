/* Pellets are the little dots that Pacman eats to gain points. They are
 * represented by a single character on the screen. */

use cgmath::{Matrix4, vec3, Transform, Point3};
use engine::graphics::color::Color;
use engine::algorithms::fill::fill_circle_inundation;

pub fn draw_small_pellet(pacman_x: f32, pacman_y: f32)
{
    // Create a data structure to represent the pellets
    struct Pellet {
        x: f32,
        y: f32,
        eaten: bool,
    }

    let mut pellets = Vec::new();

    // Fill the screen with small pellets
    for i in (0..600).step_by(40) {
        for j in (0..600).step_by(40) {
            let mut pellet = Pellet {
                x: i as f32,
                y: j as f32,
                eaten: false,
            };

            // Check if Pacman passed through the pellet
            if (pacman_x - pellet.x).abs() < 10.0 && (pacman_y - pellet.y).abs() < 10.0 {
                // Mark the pellet as "eaten"
                pellet.eaten = true;
            }

            pellets.push(pellet);
        }
    }

    // Draw the pellets that are not "eaten"
    for pellet in &pellets {
        if !pellet.eaten {
            // Don't draw the pellet if is inside ghost rectangle
            if pellet.x > 140.0 && pellet.x < 460.0 && pellet.y > 200.0 && pellet.y < 420.0 {
                continue;
            }

            // Dont draw it if is outside (left)
            if pellet.x >= 0.0 && pellet.x < 110.0 && pellet.y > 190.0 && pellet.y < 440.0 {
                continue;
            }

            // Dont draw it if is outside (right)
            if pellet.x > 490.0 && pellet.x <= 600.0 && pellet.y > 190.0 && pellet.y < 440.0 {
                continue;
            }

            fill_circle_inundation(pellet.x, pellet.y, 2.0, &Color::new(1.0, 1.0, 1.0));
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
