use crate::graphics::wrapper::{Vao, Buffer, Vertex};
use crate::algorithms::pixel::draw_pixel;
use std::ptr;

pub fn fill_rectangle(x1: f32, y1: f32, x2: f32, y2: f32)
{
    let cordenates: [f32; 12] = [
        x1, y1, 0.0, x2, y1, 0.0, x2, y2, 0.0, x1, y2, 0.0,
    ];
    let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

    let vao = Vao::new();
    vao.bind();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.buffer_data(&cordenates);

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

pub fn fill_rectangle_inundation(x1: f32, y1: f32, x2: f32, y2: f32)
{
    let mut x: f32 = x1;
    let mut y: f32 = y1;

    while x <= x2
    {
        while y <= y2
        {
            draw_pixel(x as i32, y as i32);
            y += 1.0;
        }
        y = y1;
        x += 1.0;
    }
}

pub fn fill_circle_inundation(xc: f32, yc: f32, r: f32)
{
    let mut x: f32 = 0.0;
    let mut y: f32 = r;
    let mut d: f32 = 1.25 - r;

    while x <= y
    {
        let mut i: f32 = xc - x;
        while i <= xc + x
        {
            draw_pixel(i as i32, (yc + y) as i32);
            draw_pixel(i as i32, (yc - y) as i32);
            i += 1.0;
        }
        i = xc - y;
        while i <= xc + y
        {
            draw_pixel(i as i32, (yc + x) as i32);
            draw_pixel(i as i32, (yc - x) as i32);
            i += 1.0;
        }

        if d < 0.0
        {
            d += 2.0 * x + 3.0;
        }
        else
        {
            d += 2.0 * (x - y) + 5.0;
            y -= 1.0;
        }

        x += 1.0;
    }
}

pub fn fill_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32)
{
    let vertices: [f32; 9] = [
        // Positions
        x1, y1, 0.0,
        x2, y1, 0.0,
        x3, y2, 0.0,
    ];
    let indices: [u32; 3] = [
        0, 1, 2,
    ];

    let vao = Vao::new();
    let vbo = Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    crate::algorithms::raw_attributes::set_vao_vbo(&vao, &vbo, &vertices, 3);
    let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();
    ebo.buffer_data(&indices);

    unsafe {
        gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, ptr::null());
    }
}

