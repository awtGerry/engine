/* Transformations
 * 
*/

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

pub fn rotate(x: f32, y: f32, theta: f32) -> (f32, f32)
{
    let v1: [[f32; 3]; 3] = [
        [theta.cos(), -theta.sin(), 0.0],
        [theta.sin(), theta.cos(), 0.0],
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

pub fn scale(x: f32, y: f32, sx: f32, sy: f32) -> (f32, f32)
{
    let v1: [[f32; 3]; 3] = [
        [sx, 0.0, 0.0],
        [0.0, sy, 0.0],
        [0.0, 0.0, 1.0]
    ];

    let mut v2: [f32; 3] = [x, y, 1.0];

    for i in 0..3
    {
        v2[i] = v1[i][0] * x + v1[i][1] * y + v1[i][2];
    }

    let x = &v2[0];
    let y = &v2[1];
    (*x, *y)
}
