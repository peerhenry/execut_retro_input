use std::ffi::CString;
use gl::types::*;
use rand::*;
use crate::scene::*;
use crate::shader_compiler::*;
use crate::gl_buffers::*;
use crate::gl_error_handler::*;
use crate::render_pass::*;
use crate::frame_buffer::*;

// todo rename to RetrofyScene
pub struct NoiseScene {
  program: ShaderProgram,
  vbo: GLuint,
  vao: GLuint,
  texture: GLuint,
  rand_uniform_loc: GLint,
  // pub fbo: GLuint,  // to be moved
  // rbo: GLuint,      // to be moved
  noise: Option<RenderPass>,
  extract_bright: Option<RenderPass>,
  blur_vertically: Option<RenderPass>,
  blur_horizontally_and_join: Option<RenderPass>,
  noise_fbo: Option<Framebuffer>
}

impl NoiseScene {
  pub fn new(vs_glsl: &str, fs_glsl: &str, texture: GLuint) -> Self {
    let program = build_shader_program(vs_glsl, fs_glsl).unwrap();
    NoiseScene {
      program: program,
      vbo: 0,
      vao: 0,
      texture,
      rand_uniform_loc: -1,
      // fbo: 0, // to become obs
      // rbo: 0, // to become obs
      noise: None,
      extract_bright: None,
      blur_vertically: None,
      blur_horizontally_and_join: None,
      noise_fbo: None
    }
  }
}

unsafe fn get_renderpass_locations(program_handle: GLuint) -> (GLuint, GLuint, GLuint, GLuint) {
  let noise = gl::GetSubroutineIndex(program_handle, gl::FRAGMENT_SHADER, CString::new("noise").unwrap().as_ptr());
  let extract_bright = gl::GetSubroutineIndex(program_handle, gl::FRAGMENT_SHADER, CString::new("extract_bright").unwrap().as_ptr());
  let blurVertically = gl::GetSubroutineIndex(program_handle, gl::FRAGMENT_SHADER, CString::new("blurVertically").unwrap().as_ptr());
  let blurHorizontallyAndJoin = gl::GetSubroutineIndex(program_handle, gl::FRAGMENT_SHADER, CString::new("blurHorizontallyAndJoin").unwrap().as_ptr());
  (noise, extract_bright, blurVertically, blurHorizontallyAndJoin)
}

impl Scene for NoiseScene {
  fn init(&mut self) {
    unsafe {
      self.noise = Some(RenderPass::new(self.program.handle, gl::FRAGMENT_SHADER, "noise"));
      self.extract_bright = Some(RenderPass::new(self.program.handle, gl::FRAGMENT_SHADER, "extractBright"));
      self.blur_vertically = Some(RenderPass::new(self.program.handle, gl::FRAGMENT_SHADER, "blurVertically"));
      self.blur_horizontally_and_join = Some(RenderPass::new(self.program.handle, gl::FRAGMENT_SHADER, "blurHorizontallyAndJoin"));
      self.rand_uniform_loc = gl::GetUniformLocation(self.program.handle, CString::new("baseRand").unwrap().as_ptr());

      // todo: move to Framebuffer, create in main and pass to text_scene
      /*self.fbo = make_framebuffer();
      let f_width: GLsizei = 1600; // 1920;
      let f_height: GLsizei = 900; // 1080;
      self.texture = make_frame_texture(self.fbo, f_width as _, f_height as _);
      attach_texture_to_framebuffer(self.fbo, self.texture, gl::COLOR_ATTACHMENT0);
      self.rbo = make_render_buffer(self.fbo, f_width as _, f_height as _);
      attach_renderbuffer_to_framebuffer(self.fbo, self.rbo);
      // */

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
    gl::BindTexture(gl::TEXTURE_2D, self.texture);
    gl::UseProgram(self.program.handle);
    if let Some(pass) = &self.noise { pass.set(); }
    let mut rng = rand::thread_rng();
    let rand_val: f32 = rng.gen();
    gl::Uniform1f(self.rand_uniform_loc, rand_val);
    gl::BindVertexArray(self.vao);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

    // pass extract bright colors
    /*gl::BindFramebuffer(gl::FRAMEBUFFER, self.bright_fbo);
    if let Some(pass) = &self.extract_bright { pass.set(); }
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

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
      // gl::DeleteTextures(1, &self.texture); // to become obsolet
      // gl::DeleteFramebuffers(1, &self.fbo); // to become obsolet
      // gl::DeleteRenderbuffers(1, &self.rbo); // to become obsolet
    }
  }
}