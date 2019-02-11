use std::ffi::CString;
use gl::types::*;
use rand::*;
use crate::scene::*;
use crate::shader_compiler::*;
use crate::gl_buffers::*;
use crate::gl_error_handler::*;
use crate::render_pass::*;
use crate::frame_buffer::*;
use crate::RETRO_COLOR;

pub struct RetrofyScene {
  program: ShaderProgram,
  vbo: GLuint,
  vao: GLuint,
  text_texture_handle: GLuint,
  rand_uniform_loc: GLint,
  texture_loc: GLint,
  retro_color_loc: GLint,
  line_pos_loc: GLint,
  noise: Option<RenderPass>,
  extract_bright: Option<RenderPass>,
  blur_vertically: Option<RenderPass>,
  blur_horizontally_and_join: Option<RenderPass>,
  noise_fbo: Option<Framebuffer>,
  text_texture_number: GLint,
  line_pos: GLfloat
}

impl RetrofyScene {
  pub fn new(vs_glsl: &str, fs_glsl: &str, text_texture_handle: GLuint, text_texture_number: GLint) -> Self {
    let program = build_shader_program(vs_glsl, fs_glsl).unwrap();
    RetrofyScene {
      program: program,
      vbo: 0,
      vao: 0,
      text_texture_handle,
      rand_uniform_loc: -1,
      retro_color_loc: -1,
      line_pos_loc: -1,
      noise: None,
      extract_bright: None,
      blur_vertically: None,
      blur_horizontally_and_join: None,
      noise_fbo: None,
      texture_loc: -1,
      text_texture_number,
      line_pos: 0.5
    }
  }

  pub fn update(&mut self) {
    self.line_pos = self.line_pos - 0.001;
    if self.line_pos < -0.5 { self.line_pos = 1.5; }
  }
}

impl Scene for RetrofyScene {
  fn init(&mut self) {
    unsafe {
      // fbos
      // self.noise_fbo = Some(Framebuffer::new(gl::TEXTURE23, 1600, 900));
      // render passes
      self.noise = Some(RenderPass::new(self.program.handle, gl::FRAGMENT_SHADER, "noise"));
      self.extract_bright = Some(RenderPass::new(self.program.handle, gl::FRAGMENT_SHADER, "extractBright"));
      self.blur_vertically = Some(RenderPass::new(self.program.handle, gl::FRAGMENT_SHADER, "blurVertically"));
      self.blur_horizontally_and_join = Some(RenderPass::new(self.program.handle, gl::FRAGMENT_SHADER, "blurHorizontallyAndJoin"));
      // uniforms
      self.rand_uniform_loc = gl::GetUniformLocation(self.program.handle, CString::new("baseRand").unwrap().as_ptr());
      self.texture_loc = gl::GetUniformLocation(self.program.handle, CString::new("screenTexture").unwrap().as_ptr());
      self.retro_color_loc = gl::GetUniformLocation(self.program.handle, CString::new("retroColor").unwrap().as_ptr());
      self.line_pos_loc = gl::GetUniformLocation(self.program.handle, CString::new("linePos").unwrap().as_ptr());
      // quad
      let (vbo, vao) = make_frame_quad(self.program.handle);
      self.vbo = vbo;
      self.vao = vao;
      gl_assert_ok!();
      // println!("vbo: {}, vao: {}, tex: {}, uniform: {}, fbo: {}, rbo: {}", self.vbo, self.vao, self.texture, self.rand_uniform_loc, self.fbo, self.rbo);
    }
  }

  unsafe fn draw(&self) {
    // pass noise
    // gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    if let Some(noise_fbo) = self.noise_fbo { noise_fbo.bind(); }
    else { gl::BindFramebuffer(gl::FRAMEBUFFER, 0); }
    gl::ClearColor(0.1, 0.0, 0.0, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
    gl::UseProgram(self.program.handle);
    gl::BindTexture(gl::TEXTURE_2D, self.text_texture_handle); // if not bound, we get the upside down red letters
    gl::Uniform1i(self.texture_loc, self.text_texture_number); // this should be from the text_fbo
    gl::Uniform1f(self.line_pos_loc, self.line_pos);
    gl::Uniform4fv(self.retro_color_loc, 1, RETRO_COLOR.as_ptr());
    if let Some(pass) = &self.noise { pass.set(); }
    let mut rng = rand::thread_rng();
    let rand_val: f32 = rng.gen();
    gl::Uniform1f(self.rand_uniform_loc, rand_val);
    gl::BindVertexArray(self.vao);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

    // pass extract bright colors
    /*gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    // gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
    if let Some(noise_fbo) = self.noise_fbo {
      // gl::BindFramebuffer(gl::READ_FRAMEBUFFER, noise_fbo.handle);
      gl::BindTexture(gl::TEXTURE_2D, noise_fbo.tex_handle); // without this, no noise
      gl::Uniform1i(self.texture_loc, noise_fbo.texture_number); // if this is wrong, sampler2D will just give black pixels
    } // if not bound, we get the upside down red letters
    if let Some(pass) = &self.extract_bright { pass.set(); }
    gl::DrawArrays(gl::TRIANGLES, 0, 6);*/

    /*
    // pass blur vertically
    gl::BindFramebuffer(gl::FRAMEBUFFER, self.blur_fbo);
    if let Some(pass) = &self.blur_vertically { pass.set(); }
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

    // pass blur horizontally and join
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    if let Some(pass) = &self.blur_horizontally_and_join { pass.set(); }
    gl::DrawArrays(gl::TRIANGLES, 0, 6);*/
  }

  fn cleanup(&self) {
    unsafe {
      self.program.cleanup();
      gl::DeleteBuffers(1, &self.vbo);
      gl::DeleteVertexArrays(1, &self.vao);
    }
  }
}