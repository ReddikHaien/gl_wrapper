use std::marker::PhantomData;


pub trait RawIdManager{
    fn create_resource() -> u32;
    fn delete_resource(id: u32);
}

pub struct RawId<T: RawIdManager>{
    id: u32,
    marker: PhantomData<*const T>
}

impl<T: RawIdManager> RawId<T>{
    pub fn new() -> Self{
        let id = T::create_resource();
        Self{
            id,
            marker: PhantomData
        }
    }

    pub unsafe fn from_id(id: u32) -> Self{
        Self{
            id,
            marker: PhantomData
        }
    }

    pub fn id(&self) -> u32{
        self.id
    }
}

impl<T: RawIdManager> Drop for RawId<T>{
    fn drop(&mut self) {
        T::delete_resource(self.id)
    }
}