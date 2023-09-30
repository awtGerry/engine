use std::mem;

#[derive(Debug)]
pub struct Vao {
    id: gl::types::GLuint,
}

impl Vao {
    pub fn new() -> Vao {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Vao { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub struct Buffer {
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
    usage: gl::types::GLenum,
}

impl Buffer {
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> Buffer {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Buffer { id, r#type, usage }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn buffer_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                self.usage,
            );
        }
    }
}

pub struct Vertex {
    index: gl::types::GLuint,
}

impl Vertex {
    pub fn new(
        index: gl::types::GLuint,
        size: gl::types::GLint,
        r#type: gl::types::GLenum,
        normalized: gl::types::GLboolean,
        stride: gl::types::GLsizei,
        pointer: *const gl::types::GLvoid,
    ) -> Vertex {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
            // gl::EnableVertexAttribArray(index);
        }
        Vertex { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}
