use std::{collections::HashMap, fs::File, io::Read};

use cgmath::Matrix;
use gl::types::*;

pub struct Shader {
    program: u32,
    uniform_location: HashMap<String, GLint>,
}

#[allow(temporary_cstring_as_ptr)]
impl Shader {
    pub fn new(vertex_shader: &str, fragment_shader: &str) -> Shader {
        let mut vertex_shader_file = File::open(vertex_shader)
            .unwrap_or_else(|_| panic!("No se pudo abrir el vertex: {}", vertex_shader));
        let mut fragment_shader_file = File::open(fragment_shader)
            .unwrap_or_else(|_| panic!("No se pudo abrir el shader: {}", fragment_shader));

        let mut vertex_shader_code = String::new();
        let mut fragment_shader_code = String::new();

        vertex_shader_file
            .read_to_string(&mut vertex_shader_code)
            .unwrap_or_else(|_| panic!("No se pudo leer el vertex: {}", vertex_shader));
        fragment_shader_file
            .read_to_string(&mut fragment_shader_code)
            .unwrap_or_else(|_| panic!("No se pudo leer el shader: {}", fragment_shader));

        unsafe {
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = std::ffi::CString::new(vertex_shader_code.as_bytes()).unwrap();
            gl::ShaderSource(vertex, 1, &c_str_vert.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex);

            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = std::ffi::CString::new(fragment_shader_code.as_bytes()).unwrap();
            gl::ShaderSource(fragment, 1, &c_str_frag.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment);

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);
            gl::LinkProgram(program);
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);

            Shader {
                program,
                uniform_location: HashMap::new(),
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn set_uniform(&mut self, name: &str) {
        let location = unsafe {
            gl::GetUniformLocation(
                self.program,
                std::ffi::CString::new(name).unwrap().as_ptr(),
            )
        };

        if location < 0 {
            panic!("No se pudo encontrar {}", name);
        } else {
            self.uniform_location.insert(name.to_string(), location);
        }
    }

    pub fn set_matrix4(&self, name: &str, matrix: &cgmath::Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_location[name],
                1,
                gl::FALSE,
                matrix.as_ptr(),
            );
        }
    }

}
