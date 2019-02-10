//! Example of glyph_brush usage with raw OpenGL.
//!
//! Uses instanced rendering with 1 vertex per glyph referencing a 1 byte per pixel texture.
//!
//! Controls
//! * Scroll to size text.
//! * Type to modify text.
//! * Resize window.
//! 
//! Rendering to a texture https://learnopengl.com/Advanced-OpenGL/Framebuffers
//! bloom https://learnopengl.com/Advanced-Lighting/Bloom

use gl::types::GLsizei;
use glutin::{GlWindow};
use spin_sleep::LoopHelper;

mod shader_compiler;
mod gl_buffers;
#[macro_use]
mod gl_error_handler;
mod helpers_for_glyph;
mod render_pass;

mod frame_buffer;
use frame_buffer::*;
mod scene;
use scene::*;
mod noise_scene;
use noise_scene::*;
mod text_scene;
use text_scene::*;
mod event_handler;
use event_handler::*;
mod context;
use context::*;

pub type Res<T> = Result<T, Box<std::error::Error>>;

fn main() -> Res<()> {
  let title = "glyph_brush opengl example - scroll to size, type to modify";
  let (window, mut events) = init_context(title)?;
  // INIT
  let f_width: GLsizei = 1600; // 1920;
  let f_height: GLsizei = 900; // 1080;
  let text_frame_buffer = Framebuffer::new(gl::TEXTURE0, f_width, f_height);
  let text_texture = text_frame_buffer.tex_handle;

  let mut noise_scene = NoiseScene::new(include_str!("shader/retrofy.vert.glsl"), include_str!("shader/retrofy.frag.glsl"), text_texture);
  noise_scene.init();

  let mut text_scene = TextScene::new(include_str!("shader/text.vert.glsl"), include_str!("shader/text.frag.glsl"), &window, Some(text_frame_buffer));
  text_scene.init();

  let mut loop_helper = spin_sleep::LoopHelper::builder().build_with_target_rate(250.0);
  let mut running = true;
  // RUN
  while running {
    loop_helper.loop_start();
    handle_events(&mut events, &mut running, &window, &mut text_scene)?;
    text_scene.update(&window);
    draw(&noise_scene, &text_scene, &window)?;
    update_loop_helper(&mut loop_helper, &window, title);
  }
  // CLEANUP
  text_scene.cleanup();
  noise_scene.cleanup();
  Ok(())
}

fn draw(noise_scene: &NoiseScene, text_scene: &TextScene, window: &GlWindow) -> Res<()> {
  unsafe {
    text_scene.draw();
    noise_scene.draw();
  }
  window.swap_buffers()?;
  Ok(())
}

fn update_loop_helper(loop_helper: &mut LoopHelper, window: &GlWindow, title: &str) {
  if let Some(rate) = loop_helper.report_rate() {
    window.set_title(&format!("{} {:.0} FPS", title, rate));
  }
  loop_helper.loop_sleep();
}