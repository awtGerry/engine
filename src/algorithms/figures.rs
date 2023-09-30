use crate::graphics::wrapper::{Vao, Buffer};
use crate::graphics::wrapper::*;
// use crate::algorithms::raw_attributes::set_vao_vbo;
use std::ptr;
use crate ::algorithms::lines::draw_dda_line;

/* Collection of figures that may use (or not) the algorithms
 * implemented in the algorithms module.
 */

pub fn fill_rectangle(x1: f32, y1: f32, x2: f32, y2: f32) {
    draw_dda_line(x1, y1, x2, y1);
    draw_dda_line(x2, y1, x2, y2);
    draw_dda_line(x2, y2, x1, y2);
    draw_dda_line(x1, y2, x1, y1);

    let cordenates: [f32; 12] = [
        x1, y1, 0.0, x2, y1, 0.0, x2, y2, 0.0, x1, y2, 0.0,
    ];
    let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

    let vao = Vao::new();
    vao.bind();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.buffer_data(&cordenates);

    // TODO: Las lineas se borran por lo tanto agregue un fondo blanco temporal para que se quede el rectangulo
    let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.buffer_data(&indices);

    let position_attrib = Vertex::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * std::mem::size_of::<f32>() as i32,
        ptr::null(),
    );
    position_attrib.enable();

    unsafe {
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()); // Agregar fondo blanco
    }
}
