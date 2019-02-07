use std::{
    env,
    ffi::CString,
    io::{self, Write},
    mem, ptr, 
};
use gl::types::*;
use super::Res;

pub unsafe fn make_framebuffer() -> GLuint {
  let mut fbo: GLuint = 0;
  gl::GenFramebuffers(1, &mut fbo);
  fbo
}

pub unsafe fn make_frame_texture(fbo: GLuint, width: GLsizei, height: GLsizei) -> GLuint {
  let mut tbo: GLuint = 0;
  gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
  gl::GenTextures(1, &mut tbo);
  gl::BindTexture(gl::TEXTURE_2D, tbo);
  gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as _, width, height, 0, gl::RGB as _, gl::UNSIGNED_BYTE, ptr::null());
  gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _); // param min filter
  gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _); // param mag filter
  gl::BindTexture(gl::TEXTURE_2D, 0);
  gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
  tbo
}

pub unsafe fn attach_texture_to_framebuffer(fbo: GLuint, tbo: GLuint, color_attachment: GLenum) {
  gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
  gl::FramebufferTexture2D(gl::FRAMEBUFFER, color_attachment, gl::TEXTURE_2D, tbo, 0); // attach texture 
  gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
}

// framebuffer must be bound for this one
pub unsafe fn make_render_buffer(fbo: GLuint, width: GLsizei, height: GLsizei) -> GLuint {
  gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
  let mut rbo: GLuint = 0;
  gl::GenRenderbuffers(1, &mut rbo);
  gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
  gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width, height);
  gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
  gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
  rbo
}

pub unsafe fn complete_framebuffer(framebuffer: GLuint, rbo: GLuint) {
  gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
  gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);
  if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
    panic!("framebuffer error");
  }
  gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
}

pub unsafe fn setup_attribs(frame_program: GLuint, vertex_byte_size: GLsizei, use_divisor: bool, attribs: &[(&str, i32)]) -> Res<()> {
  // set attributes
  let mut offset = 0;
  println!("vertex_byte_size {}", vertex_byte_size); // DEBUG
  for (v_field, float_count) in attribs {
    let attr = gl::GetAttribLocation(frame_program, CString::new(*v_field)?.as_ptr());
    // println!("setting up attrib {}, {}, {}", v_field, float_count, attr); // DEBUG
    if attr < 0 {
      return Err(format!("{} GetAttribLocation -> {}", v_field, attr).into());
    }
    // println!("we calling VertexAttribPointer {}, {}, {}, {}", attr as GLuint, *float_count, vertex_byte_size, offset); // DEBUG
    gl::VertexAttribPointer(
      attr as _,        // location
      *float_count,     // size
      gl::FLOAT,        // type
      gl::FALSE as _,   // normalized
      vertex_byte_size, // stride
      offset as _,      // offset
    );
    gl::EnableVertexAttribArray(attr as _);
    if use_divisor { gl::VertexAttribDivisor(attr as _, 1); }
    offset += float_count * 4;
  }
  Ok(())
}

pub unsafe fn make_frame_quad(frame_program: GLuint) -> (GLuint, GLuint) {
    let vdata: Vec<GLfloat> = vec![
    // X    Y     U   V
    -1.0,  1.0, 0.0, 1.0,
     1.0,  1.0, 1.0, 1.0,
    -1.0, -1.0, 0.0, 0.0,
    -1.0, -1.0, 0.0, 0.0,
     1.0,  1.0, 1.0, 1.0,
     1.0, -1.0, 1.0, 0.0
  ];
  let vertices: Vec<GLfloat> = vdata.into_iter().map(|x| x).collect();
  let buffer_byte_size = (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
  // gen
  let mut vao: GLuint = 0;
  let mut vbo: GLuint = 0;
  gl::GenVertexArrays(1, &mut vao);
  gl::GenBuffers(1, &mut vbo);
  // bind
  gl::BindVertexArray(vao);
  gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
  // send
  gl::BufferData(
                gl::ARRAY_BUFFER,
                buffer_byte_size,
                vertices.as_ptr() as _,
                gl::STATIC_DRAW,
              );
  let vertex_size: GLsizei = mem::size_of::<[GLfloat; 4]>() as GLsizei;
  setup_attribs(frame_program, vertex_size, false, &[("aPos", 2), ("aTexCoords", 2)]).unwrap(); // ARRAY_BUFFER must also be bound for this one
  // unbind buffers
  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  gl::BindVertexArray(0);
  (vbo, vao)
}