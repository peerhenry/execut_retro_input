use gl::types::*;
use std::ptr;

#[derive(Debug, Copy, Clone)]
pub struct Framebuffer {
  handle: GLuint,
  pub tex_handle: GLuint
}

impl Framebuffer {
  // texture unit is gl::TEXTURE0 or gl::TEXTURE1 etc.
  pub fn new(texture_unit: GLenum, width: GLsizei, height: GLsizei) -> Self {
    let mut handle: GLuint = 0;
    let mut tex_handle: GLuint = 0;
    unsafe {
      // gen buffer
      gl::GenFramebuffers(1, &mut handle);
      gl::BindFramebuffer(gl::FRAMEBUFFER, handle);
      // create texture
      gl::GenTextures(1, &mut tex_handle);
      gl::ActiveTexture(texture_unit);
      gl::BindTexture(gl::TEXTURE_2D, tex_handle);
      gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as _, width, height, 0, gl::RGB as _, gl::UNSIGNED_BYTE, ptr::null());
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _); // param min filter
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _); // param mag filter
      // attach texture
      gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex_handle, 0);
      // set target
      let draw_buffers = vec![gl::COLOR_ATTACHMENT0];
      gl::DrawBuffers(1, draw_buffers.as_ptr() as _);
      // unbind
      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
    Framebuffer {
      handle,
      tex_handle
    }
  }

  pub unsafe fn bind(&self) {
    gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle);
  }

  pub unsafe fn cleanup(&self) {
    gl::DeleteTextures(1, &self.tex_handle);
    gl::DeleteFramebuffers(1, &self.handle);
  }
}