use std::{collections::BTreeMap, rc::Rc, ffi::c_void};

use crate::{buffer::Buffer, internal::{RawId, RawIdManager}, shader::{Program, UniformContainer}};

#[derive(Clone)]
pub struct VertexArray{
    pointers: BTreeMap<u32, Buffer>,
    id: Rc<RawId<Self>>,
    indices: Option<Buffer>,
    draw_mode: u32,
    count: i32,
}

impl VertexArray{
    pub fn new() -> Self{
        Self{
            pointers: BTreeMap::new(),
            id: Rc::new(RawId::new()),
            indices: None,
            draw_mode: gl::POINTS,
            count: 0
        }
    }

    pub fn set_pointer(
        &mut self,
        pointer: u32,
        buffer: Buffer,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        offset: u32
    ){
        unsafe{
            gl::BindVertexArray(self.id.id());
            buffer.bind();
            gl::VertexAttribPointer(
                pointer,
                size,
                type_,
                match normalized { true => gl::TRUE, _ => gl::FALSE },
                stride,
                offset as *const c_void
            );
            gl::EnableVertexAttribArray(pointer);
            buffer.unbind();
            self.pointers.insert(pointer, buffer);
        }
    }

    pub fn draw(&self, program: &Program, uniforms: &dyn UniformContainer){
        program.bind();
        uniforms.bind();
        self.bind();
        unsafe{
            match &self.indices{
                Some(buffer) => {
                    buffer.bind();
                gl::DrawElements(self.draw_mode, self.count, gl::UNSIGNED_SHORT, 0 as *const c_void);
                    buffer.unbind();
                },
                None => {
                    gl::DrawArrays(self.draw_mode, 0, self.count);
                },
            }
        }
        
    }


    pub fn remove_pointer(&mut self, pointer: u32){
        self.pointers.remove(&pointer);
    }

    pub fn add_indices(&mut self, buffer: Buffer, count: i32){
        if buffer.target() != gl::ELEMENT_ARRAY_BUFFER{
            panic!("Buffer is not an element buffer!");
        }
        self.bind();
        buffer.bind();
        self.unbind();
        buffer.unbind();
        
        self.indices = Some(buffer);
        self.count = count;
    }

    pub fn set_count(&mut self, count: i32){
        self.count = count;
    }

    pub fn set_draw_mode(&mut self, mode: u32){
        self.draw_mode = mode;
    }

    pub fn remove_indices(&mut self){
        self.indices = None;
    }

    pub(crate) fn bind(&self){
        unsafe{
            gl::BindVertexArray(self.id.id());
        }
    }

    pub(crate) fn unbind(&self){
        unsafe{
            gl::BindVertexArray(0);
        }
    }
}



impl RawIdManager for VertexArray{
    fn create_resource() -> u32 {
        unsafe{
            let mut i = 0;
            gl::CreateVertexArrays(1, &mut i);
            i
        }
    }

    fn delete_resource(id: u32) {
        unsafe{
            gl::DeleteVertexArrays(1, &id);
        }
    }
}