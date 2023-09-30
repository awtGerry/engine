use engine::graphics::shader::Shader;

extern crate gl;
use gl::types::*;

extern crate image;
use image::GenericImage;

use std::path::Path;
use std::os::raw::c_void;
use std::mem;
use std::ptr;

#[allow(non_snake_case)]
pub fn bg_texture() -> (Shader, GLuint, GLuint, GLuint, GLuint) {
    let (new_shader, VBO, VAO, EBO, texture) = unsafe {
        // build and compile our shader program
        // ------------------------------------
        let new_shader = Shader::new(
            "clock/shaders/bg.shader.vs",
            "clock/shaders/bg.shader.fs",
        );

        let vertices: [f32; 32] = [
            // positions       // colors        // texture coords
             1.0,  1.0, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
             1.0, -1.0, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
            -1.0, -1.0, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
            -1.0,  1.0, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
        ];
        let indices = [
            0, 1, 3,
            1, 2, 3
        ];
        let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);

        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // color attribute
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
        // texture coord attribute
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2);

        // load and create a texture
        // -------------------------
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new("clock/images/all_together180.jpg")).expect("Failed to load texture");
        let data = img.raw_pixels();
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGB as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0,
                       gl::RGB,
                       gl::UNSIGNED_BYTE,
                       &data[0] as *const u8 as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        (new_shader, VBO, VAO, EBO, texture)
    };
    (new_shader, VBO, VAO, EBO, texture)
}
