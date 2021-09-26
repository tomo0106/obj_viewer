use gl::types::{GLenum, GLfloat, GLint, GLsizei, GLsizeiptr};
use std::mem;
use std::os::raw::c_void;

pub struct Vertex {
    vao: u32,
    _vbo: u32,
    ibo: u32,
    vertex_num: i32,
}

impl Vertex {
    pub fn new(
        size: GLsizeiptr,
        vertices: *const c_void,
        usage: GLenum,
        attribute_type_vec: std::vec::Vec<GLenum>,
        attribute_size_vec: std::vec::Vec<GLint>,
        stride: GLsizei,
        vertex_num: i32,
    ) -> Vertex {
        let mut vao = 0;
        let mut vbo = 0;
        unsafe {
            // create vertex array object and vertex buffer object
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // bind buffer
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size, vertices, usage);

            let mut offset = 0;
            for i in 0..attribute_type_vec.len() {
                // attribute属性を有効にする
                gl::EnableVertexAttribArray(i as u32);
                // attribute属性を登録
                gl::VertexAttribPointer(
                    i as u32,
                    attribute_size_vec[i],
                    attribute_type_vec[i],
                    gl::FALSE,
                    stride,
                    (offset * mem::size_of::<GLfloat>()) as *const c_void,
                );
                offset += attribute_size_vec[i] as usize;
            }

            // unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        Vertex {
            vao: vao,
            _vbo: vbo,
            ibo: 0u32,
            vertex_num: vertex_num,
        }
    }

    pub fn setup_ibo(&mut self, indices_size: GLsizeiptr, indices: *const c_void) {
        unsafe {
            gl::GenBuffers(1, &mut self.ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                indices_size,
                indices,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    #[allow(dead_code)]
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            // gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_num);
            gl::DrawArrays(gl::POINTS, 0, self.vertex_num);
            gl::PointSize(10.0);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_elements(
        &self,
        mode: gl::types::GLenum,
        indices_size: GLsizei,
        indices: *const c_void,
    ) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::BindVertexArray(self.vao); //add
            gl::DrawElements(mode, indices_size, gl::UNSIGNED_INT, indices);
            gl::BindVertexArray(0); // add
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    // pub fn draw_elements2(&self, indices: &Indices) {
    //     unsafe {
    //         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
    //         gl::BindVertexArray(self.vao); //add
    //         let mut counter = 0;
    //         for i in 0..indices.polygon_count.len() {
    //             match indices.polygon_count[i] {
    //                 2 => {
    //                     gl::DrawElements(
    //                         gl::LINES,
    //                         indices.polygon_count[i] as GLsizei,
    //                         gl::UNSIGNED_INT,
    //                         indices.vertex_indices[counter..].as_ptr() as *const c_void,
    //                     );
    //                     counter += 2;
    //                 }
    //                 3 => {
    //                     gl::DrawElements(
    //                         gl::TRIANGLES,
    //                         indices.polygon_count[i] as GLsizei,
    //                         gl::UNSIGNED_INT,
    //                         indices.vertex_indices[counter..].as_ptr() as *const c_void,
    //                     );
    //                     counter += 3;
    //                 }
    //                 4 => {
    //                     gl::DrawElements(
    //                         gl::QUADS,
    //                         indices.polygon_count[i] as GLsizei,
    //                         gl::UNSIGNED_INT,
    //                         indices.vertex_indices[counter..].as_ptr() as *const c_void,
    //                     );
    //                     counter += 4;
    //                 }
    //                 _ => (),
    //             }
    //         }
    //         gl::BindVertexArray(0); // add
    //         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    //     }
    // }
}
