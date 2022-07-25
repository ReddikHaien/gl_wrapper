pub use gl;
pub(crate) mod internal;
pub mod buffer;
pub mod vao;
pub mod shader;

use std::os::raw::c_void;

pub fn load_with<F>(x: F)
    where F: FnMut(&'static str) -> *const c_void
{
    gl::load_with(x);
}