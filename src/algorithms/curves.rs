use crate::graphics::wrapper::{Vao, Buffer};

use crate::algorithms::pixel::draw_pixel;
use crate::algorithms::raw_attributes::set_vao_vbo;

#[allow(unused)]
pub fn infinity_symbol(points: [f32]) {
    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);

    set_vao_vbo(&vao, &vbo, &points, 2);

    /* Algorithm goes here */
}
