use std::{
    env,
    ffi::CString,
    io::{self, Write},
    mem, ptr, 
};
use gl::types::*;
use rand::*;
use crate::scene::*;
use crate::shader_compiler::*;
use crate::gl_buffers::*;
use crate::gl_error_handler::*;

pub struct NoiseScene {
  program: ShaderProgram,
  vbo: GLuint,
  vao: GLuint,
  texture: GLuint,
  rand_uniform_loc: GLint,
  pub fbo: GLuint,
  rbo: GLuint
}

impl Scene for NoiseScene {
  fn new(vs_glsl: &str, fs_glsl: &str) -> Self {
    let program = build_shader_program(vs_glsl, fs_glsl).unwrap();
    NoiseScene {
      program: program,
      vbo: 0,
      vao: 0,
      texture: 0,
      rand_uniform_loc: -1,
      fbo: 0,
      rbo: 0
    }
  }

  fn init(&mut self) {
    unsafe {
      self.rand_uniform_loc = gl::GetUniformLocation(self.program.handle, CString::new("baseRand").unwrap().as_ptr());
      self.fbo = make_framebuffer();
      let f_width: GLsizei = 1600; // 1920;
      let f_height: GLsizei = 900; // 1080;
      self.texture = make_frame_texture(self.fbo, f_width as _, f_height as _);
      attach_texture_to_framebuffer(self.fbo, self.texture, gl::COLOR_ATTACHMENT0);
      self.rbo = make_render_buffer(self.fbo, f_width as _, f_height as _);
      complete_framebuffer(self.fbo, self.rbo);
      let (vbo, vao) = make_frame_quad(self.program.handle);
      self.vbo = vbo;
      self.vao = vao;
      gl_assert_ok!();
      println!("vbo: {}, vao: {}, tex: {}, uniform: {}, fbo: {}, rbo: {}", self.vbo, self.vao, self.texture, self.rand_uniform_loc, self.fbo, self.rbo);
    }
  }

  fn update(&self) {

  }

  fn draw(&self) {
    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
      gl::ClearColor(0.1, 0.0, 0.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);
      gl::BindTexture(gl::TEXTURE_2D, self.texture);
      gl::UseProgram(self.program.handle);
      let mut rng = rand::thread_rng();
      let rand_val: f32 = rng.gen();
      gl::Uniform1f(self.rand_uniform_loc, rand_val);
      gl::BindVertexArray(self.vao);
      gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }
  }

  fn cleanup(&self) {
    unsafe {
      gl::DeleteProgram(self.program.handle);
      gl::DeleteShader(self.program.fragment_shader);
      gl::DeleteShader(self.program.vertex_shader);
      gl::DeleteBuffers(1, &self.vbo);
      gl::DeleteVertexArrays(1, &self.vao);
      gl::DeleteTextures(1, &self.texture);
      gl::DeleteFramebuffers(1, &self.fbo);
      gl::DeleteRenderbuffers(1, &self.rbo);
    }
  }
}