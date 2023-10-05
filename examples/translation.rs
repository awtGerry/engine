use cgmath::{Matrix4, SquareMatrix, vec3, Rad, Matrix};
use engine::graphics::window::Window;
use engine::graphics::wrapper::*;
// use engine::graphics::shader::Shader;
use engine::graphics::l_shader::Shader;
use engine::algorithms::figures::draw_rectangle;
use engine::algorithms::raw_attributes::set_vao_vbo;
use gl::types::{GLsizeiptr, GLfloat, GLsizei};

use std::ffi::{CString, CStr};

use std::{ptr, mem, os::raw::c_void};

fn main() {
    let mut window = Window::new(1000, 768, "Translation");

    window.init();

    let (shaders, vbo, vao, ebo, texture1, texture2) = unsafe {
        // build and compile our shader program
        // ------------------------------------
        let shader = Shader::new("examples/shaders/translation.vs", "examples/shaders/translation.fs");

        let vertices: [f32; 20] = [
            // positions       // texture coords
             0.2,  0.2, 0.0,   1.0, 1.0, // top right
             0.2, -0.2, 0.0,   1.0, 0.0, // bottom right
            -0.2, -0.2, 0.0,   0.0, 0.0, // bottom left
            -0.2,  0.2, 0.0,   0.0, 1.0  // top left
        ];
        let indices = [
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // texture coord attribute
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);

        // load and create a texture
        // -------------------------
        let (mut texture1, mut texture2) = (0, 0);

        // texture 1
        // ---------
        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1);
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // texture 2
        // ---------
        gl::GenTextures(1, &mut texture2);
        gl::BindTexture(gl::TEXTURE_2D, texture2);
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        shader.useProgram();

        let name = CStr::from_bytes_with_nul(b"texture1\0").unwrap();
        shader.setInt(&name, 0);
        let name = CStr::from_bytes_with_nul(b"texture2\0").unwrap();
        shader.setInt(&name, 1);

        (shader, vbo, vao, ebo, texture1, texture2)
    };

    let mut scale: f32 = 0.0;
    
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            let mut transform: Matrix4<f32> = Matrix4::identity();
            scale+=0.001;
            transform = transform * Matrix4::<f32>::from_translation(vec3(scale, scale, 0.0));

            shaders.useProgram();
            let transform_loc = gl::GetUniformLocation(shaders.ID, "transform".as_ptr() as *const gl::types::GLchar);
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform.as_ptr());

            // render container
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        window.update();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
    }
}
