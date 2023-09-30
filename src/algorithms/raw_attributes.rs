use crate::graphics::wrapper::*;
use crate::graphics::wrapper::{Vao, Buffer};

use gl::types::GLsizei;
use std::ptr;

pub fn set_vao_vbo(vao: &Vao, vbo: &Buffer, vertices: &[f32], size: usize) {
    vao.bind();
    vbo.bind();
    vbo.buffer_data(vertices);

    let position_attrib_location = Vertex::new(
        0,
        size as i32,
        gl::FLOAT,
        gl::FALSE,
        (size * std::mem::size_of::<f32>()) as GLsizei,
        ptr::null(),
    );
    position_attrib_location.enable();
}
