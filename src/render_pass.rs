use std::ffi::CString;
use gl::types::*;

pub struct RenderPass {
  shaderType: GLenum,
  handle: GLuint
}

impl RenderPass {
  pub unsafe fn new(program_handle: GLuint, shaderType: GLenum, name: &str) -> Self {
    let handle = gl::GetSubroutineIndex(program_handle, gl::FRAGMENT_SHADER, CString::new(name).unwrap().as_ptr());
    RenderPass {
      shaderType,
      handle
    }
  }

pub unsafe fn set(&self) {
    gl::UniformSubroutinesuiv(self.shaderType, 1, &self.handle);
  }
}