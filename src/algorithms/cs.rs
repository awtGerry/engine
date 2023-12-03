// Purpose: Contains functions for drawing 3D shapes.
use crate::graphics::color::Color;
use crate::algorithms::lines::draw_dda_line_color;
use crate::algorithms::pixel::draw_pixel_color;

/* Draw a 3D cube using projections
 * To make 3D work, we need to use a projection matrix
 * This is a matrix that will transform 3D coordinates into 2D coordinates
 * 
 * @param points: The points of the cube
 * @param vp: The view point
 * @param color: The color of the cube
 */
pub fn projection_cube(points: &Vec<Vec<f32>>, vp: &[f32; 3], color: &Color)
{
    // Set up the projection matrix
    let mut projection: Vec<Vec<f32>> = vec![vec![0.0; 2]; 8];

    // Apply the projection matrix to each point
    for i in 0..points.len()
    {
        let u = (-vp[2]) / (points[i][2] - vp[2]);
        projection[i][0] = points[i][0] + (vp[0] * u);
        projection[i][1] = points[i][1] + (vp[1] * u);
    }

    // Draw the borders of the cube
    for i in 0..8
    {
        let x = projection[i][0] as i32;
        let y = projection[i][1] as i32;
        draw_pixel_color(x, y, &color);
    }

    // Define the faces of the cube
    let faces = vec![
        vec![0, 2, 6, 4],
        vec![5, 7, 3, 1],
        vec![4, 5, 1, 0],
        vec![6, 7, 3, 2],
        vec![0, 1, 3, 2],
        vec![4, 5, 7, 6],
    ];

    // Draw the faces of the cube
    for face in faces
    {
        draw_dda_line_color(projection[face[0]][0], projection[face[0]][1], projection[face[1]][0], projection[face[1]][1], &color);
        draw_dda_line_color(projection[face[1]][0], projection[face[1]][1], projection[face[2]][0], projection[face[2]][1], &color);
        draw_dda_line_color(projection[face[2]][0], projection[face[2]][1], projection[face[3]][0], projection[face[3]][1], &color);
        draw_dda_line_color(projection[face[3]][0], projection[face[3]][1], projection[face[0]][0], projection[face[0]][1], &color);
    }
}
