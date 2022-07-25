use std::{rc::Rc, mem::size_of};

use crate::internal::{RawId, RawIdManager};

#[derive(Clone)]
pub struct Buffer{
    target: u32,
    id: Rc<RawId<Self>>
}

impl Buffer{
    pub fn new(target: u32) -> Self{
        Self { target, id: Rc::new(RawId::new()) }
    }

    pub fn set_data<T>(&self, data: &[T], usage: u32){
        unsafe{
            self.bind();
            gl::BufferData(
                self.target,
                (size_of::<T>() * data.len()) as isize,
                data.as_ptr().cast(),
                usage
            );
            self.unbind();
        }
    }

    pub fn target(&self) -> u32{
        self.target
    }

    pub(crate) fn bind(&self){
        unsafe{
            gl::BindBuffer(self.target,self.id.id());
        }
    }

    pub(crate) fn unbind(&self){
        unsafe{
            gl::BindBuffer(self.target, 0);
        }
    }
}

impl RawIdManager for Buffer{
    fn create_resource() -> u32 {
        unsafe{
            let mut i = 0;
            gl::CreateBuffers(1,&mut i);
            i
        }
    }

    fn delete_resource(id: u32) {
        unsafe{
            gl::DeleteBuffers(1, &id);
        }
    }
}