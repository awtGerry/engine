use crate::algorithms::pixel::draw_pixel;

#[allow(unused)]
pub fn ellipse(xc: f32, yc: f32, rx: f32, ry: f32) {
    let mut x = 0.0;
    let mut y = ry;

    let mut d1 = (ry.powi(2) - rx.powi(2) * ry + 0.25 * rx.powi(2)) as f32;
    let mut dx = 2.0 * ry.powi(2) * x;
    let mut dy = 2.0 * rx.powi(2) * y;

    while dx < dy {
        draw_pixel(xc as i32 + x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 + x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 - y as i32);

        if d1 < 0.0 {
            x += 1.0;
            dx += 2.0 * ry.powi(2);
            d1 += dx + ry.powi(2);
        } else {
            x += 1.0;
            y -= 1.0;
            dx += 2.0 * ry.powi(2);
            dy -= 2.0 * rx.powi(2);
            d1 += dx - dy + ry.powi(2);
        }
    }

    let mut d2 = ((ry.powi(2) * (x + 0.5).powi(2)) + (rx.powi(2) * (y - 1.0).powi(2)) - rx.powi(2) * ry.powi(2)) as f32;

    while y >= 0.0 {
        draw_pixel(xc as i32 + x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 + y as i32);
        draw_pixel(xc as i32 + x as i32, yc as i32 - y as i32);
        draw_pixel(xc as i32 - x as i32, yc as i32 - y as i32);

        if d2 > 0.0 {
            y -= 1.0;
            dy -= 2.0 * rx.powi(2);
            d2 += rx.powi(2) - dy;
        } else {
            y -= 1.0;
            x += 1.0;
            dx += 2.0 * ry.powi(2);
            dy -= 2.0 * rx.powi(2);
            d2 += dx - dy + rx.powi(2);
        }

    }
}
