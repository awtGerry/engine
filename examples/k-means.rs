/*
   K-means clustering algorithm
   this is a simple implementation of the k-means clustering algorithm, it will
   be implemented in rust using opengl later on.
*/

use engine::graphics::color::Color;
use engine::graphics::window::Window;
use engine::algorithms::fill::fill_circle_inundation;

const WIDTH: f32 = 1366.0;
const HEIGHT: f32 = 768.0;

#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Cluster {
    points: Vec<Point>,
    centroid: Point,
}

// Function to calculate the distance between two points
// It will return a vector with the distance between the two points
fn kmeans() -> Vec<(f32, f32)>
{
    let mut points: Vec<Point> = Vec::new();
    let mut clusters: Vec<Cluster> = Vec::new();

    // Create 2000 random points
    for _ in 0..1000
    {
        let x = rand::random::<f32>() * WIDTH;
        let y = rand::random::<f32>() * HEIGHT;
        points.push(Point { x, y });
    }

    // Create 12 random clusters
    for _ in 0..6
    {
        let x = rand::random::<f32>() * WIDTH;
        let y = rand::random::<f32>() * HEIGHT;
        clusters.push(Cluster {
            points: Vec::new(),
            centroid: Point { x, y },
        });
    }

    // Iterate 100 times
    for _ in 0..100
    {
        // Assign the points
        for point in &points
        {
            let mut distance = std::f32::MAX;
            let mut cluster_id = 0;

            for (id, cluster) in clusters.iter().enumerate()
            {
                let dx = point.x - cluster.centroid.x;
                let dy = point.y - cluster.centroid.y;
                let d = dx * dx + dy * dy;

                if d < distance
                {
                    distance = d;
                    cluster_id = id;
                }
            }

            clusters[cluster_id].points.push(point.clone());
        }

        // Update centroids
        for cluster in &mut clusters
        {
            let mut x = 0.0;
            let mut y = 0.0;

            for point in &cluster.points
            {
                x += point.x;
                y += point.y;
            }

            cluster.centroid.x = x / cluster.points.len() as f32;
            cluster.centroid.y = y / cluster.points.len() as f32;
        }
    }

    let mut result: Vec<(f32, f32)> = Vec::new();

    for cluster in &clusters
    {
        result.push((cluster.centroid.x, cluster.centroid.y));
    }

    result
}

fn main()
{
    let mut window = Window::new(WIDTH, HEIGHT, "K-means");
    window.init();

    while !window.should_close()
    {
        unsafe
        {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            // gl::Clear(gl::COLOR_BUFFER_BIT);
            let v = kmeans();
            for (x, y) in &v
            {
                fill_circle_inundation(*x, *y, 3.0, &Color::new(0.0, 1.0, 0.0));
            }
        }
        window.update();
    }

}
