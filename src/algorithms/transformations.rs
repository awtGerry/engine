#[allow(unused_imports)]
use crate::graphics::wrapper::{Vao, Buffer};

#[allow(unused_imports)]
use crate::algorithms::pixel::draw_pixel;
#[allow(unused_imports)]
use crate::algorithms::raw_attributes::set_vao_vbo;

#[allow(unused_imports)]
use super::lines::draw_dda_line;

pub fn rotate(x: &f32, y: &f32, angle: &f32) -> (f32, f32) {
    let x1 = x * angle.cos() - y * angle.sin();
    let y1 = x * angle.sin() + y * angle.cos();
    (x1, y1)
}
