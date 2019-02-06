//! Example of glyph_brush usage with raw OpenGL.
//!
//! Uses instanced rendering with 1 vertex per glyph referencing a 1 byte per pixel texture.
//!
//! Controls
//! * Scroll to size text.
//! * Type to modify text.
//! * Resize window.

use gl::types::*;
use glutin::{Api, GlContext, GlProfile, GlRequest};
use glyph_brush::{rusttype::*, *};
use std::{
    env,
    ffi::CString,
    io::{self, Write},
    mem, ptr, 
};
mod helpers;
use helpers::*;

pub type FrameVertex = [GLfloat; 4];

#[macro_use]
macro_rules! gl_assert_ok {
  () => {{
    let err = gl::GetError();
    assert_eq!(err, gl::NO_ERROR, "gl error: {}", gl_err_to_str(err));
  }};
}

fn main() -> Res<()> {
  env_logger::init();

  if cfg!(target_os = "linux") {
      // winit wayland is currently still wip
      if env::var("WINIT_UNIX_BACKEND").is_err() {
          env::set_var("WINIT_UNIX_BACKEND", "x11");
      }
      // disables vsync sometimes on x11
      if env::var("vblank_mode").is_err() {
          env::set_var("vblank_mode", "0");
      }
  }

  let mut events = glutin::EventsLoop::new();
  let title = "glyph_brush opengl example - scroll to size, type to modify";

  let window = glutin::GlWindow::new(
      glutin::WindowBuilder::new()
          .with_dimensions((1024, 576).into())
          .with_title(title),
      glutin::ContextBuilder::new()
          .with_gl_profile(GlProfile::Core)
          .with_gl(GlRequest::Specific(Api::OpenGl, (4, 5))) // was 3.2
          .with_srgb(true),
      &events,
  )?;
  unsafe { window.make_current()? };

  let font_bytes: &[u8] = include_bytes!("../fonts/retro computer_demo.ttf");
  let mut glyph_brush = GlyphBrushBuilder::using_font_bytes(font_bytes).build();

  // Load the OpenGL function pointers
  gl::load_with(|symbol| window.get_proc_address(symbol) as _);

  // setup program 1: Frame program
  
  let mut using_frame_buffer = false;
  let frame_vs = compile_shader(include_str!("shader/frame.vert.glsl"), gl::VERTEX_SHADER)?;
  let frame_fs = compile_shader(include_str!("shader/frame.frag.glsl"), gl::FRAGMENT_SHADER)?;
  let frame_program = link_program(frame_vs, frame_fs)?;
  let f_width: GLsizei = 1920;
  let f_height: GLsizei = 1080;
  let mut fbo: GLuint = 0;
  let mut rbo: GLuint = 0;
  let mut frame_vbo: GLuint = 0;
  let mut frame_vao: GLuint = 0;
  let mut frame_texture: GLuint = 0;
  unsafe {
    fbo = make_framebuffer();
    frame_texture = make_frame_texture(fbo, f_width, f_height);
    attach_texture_to_framebuffer(fbo, frame_texture);
    rbo = make_render_buffer(fbo, f_width, f_height);
    complete_framebuffer(fbo, rbo);
    let tup = make_frame_quad(frame_program);
    frame_vbo = tup.0;
    frame_vao = tup.1;
    gl_assert_ok!();
    using_frame_buffer = true;
  }

  
  println!("time to setup font shaders"); // DEBUG
  // setup program 2: Font shaders
  let vs = compile_shader(include_str!("shader/vert.glsl"), gl::VERTEX_SHADER)?;
  let fs = compile_shader(include_str!("shader/frag.glsl"), gl::FRAGMENT_SHADER)?;
  let program = link_program(vs, fs)?;

  let mut vao = 0;
  let mut vbo = 0;
  let mut glyph_texture = 0;

  unsafe {
    // Create Vertex Array Object
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    // Create a Vertex Buffer Object
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

    {
      // Create a texture for the glyphs
      // The texture holds 1 byte per pixel as alpha data
      gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
      gl::GenTextures(1, &mut glyph_texture);
      gl::BindTexture(gl::TEXTURE_2D, glyph_texture);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
      let (width, height) = glyph_brush.texture_dimensions();
      gl::TexImage2D(
          gl::TEXTURE_2D,
          0,
          gl::RED as _,
          width as _,
          height as _,
          0,
          gl::RED,
          gl::UNSIGNED_BYTE,
          ptr::null(),
      );
      gl_assert_ok!();
    }

    // Use shader program
    gl::UseProgram(program);
    gl::BindFragDataLocation(program, 0, CString::new("out_color")?.as_ptr());

    // Specify the layout of the vertex data
    setup_attribs(
      program, 
      mem::size_of::<VertexForGlyph>() as _, 
      true,
      &[
        ("left_top", 3),
        ("right_bottom", 2),
        ("tex_left_top", 2),
        ("tex_right_bottom", 2),
        ("color", 4),
      ]
    )?;

    // Enabled alpha blending
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    // Use srgb for consistency with other examples
    gl::Enable(gl::FRAMEBUFFER_SRGB);
    gl::ClearColor(0.02, 0.02, 0.02, 1.0);
    // vao is used after this somewhere...
  }
  println!("font hocus pocus is done"); // DEBUG
  // */

  let mut text: String = include_str!("text/lipsum.txt").into();
  let mut font_size: f32 = 18.0;

  let mut loop_helper = spin_sleep::LoopHelper::builder().build_with_target_rate(250.0);
  let mut running = true;
  let mut vertex_count = 0;
  let mut vertex_max = vertex_count;
  let mut dimensions = window
      .get_inner_size()
      .ok_or("get_inner_size = None")?
      .to_physical(window.get_hidpi_factor());

  // println!("Time to enter events loop"); // DEBUG
  while running {
    loop_helper.loop_start();
    events.poll_events(|event| {
      use glutin::*;
      if let Event::WindowEvent { event, .. } = event {
        match event {
          WindowEvent::CloseRequested => running = false,
          WindowEvent::Resized(size) => {
            let dpi = window.get_hidpi_factor();
            window.resize(size.to_physical(dpi));
            if let Some(ls) = window.get_inner_size() {
              dimensions = ls.to_physical(dpi);
              unsafe {
                gl::Viewport(0, 0, dimensions.width as _, dimensions.height as _);
              }
            }
          }
          WindowEvent::KeyboardInput {
            input:
              KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(keypress),
                ..
              },
            ..
          } => match keypress {
            VirtualKeyCode::Escape => running = false,
            VirtualKeyCode::Back => {
              text.pop();
            }
            _ => (),
          },
          WindowEvent::ReceivedCharacter(c) => {
            if c != '\u{7f}' && c != '\u{8}' {
              text.push(c);
            }
          }
          WindowEvent::MouseWheel {
            delta: MouseScrollDelta::LineDelta(_, y),
            ..
          } => {
            // increase/decrease font size
            let old_size = font_size;
            let mut size = font_size;
            if y > 0.0 {
              size += (size / 4.0).max(2.0)
            } else {
                size *= 4.0 / 5.0
            };
            font_size = size.max(1.0).min(2000.0);
            if (font_size - old_size).abs() > 1e-2 {
              eprint!("\r                            \r");
              eprint!("font-size -> {:.1}", font_size);
              let _ = io::stderr().flush();
            }
          }
          _ => {}
        }
      }
    });
    
    let width = dimensions.width as f32;
    let height = dimensions.height as _;
    let scale = Scale::uniform((font_size * window.get_hidpi_factor() as f32).round());
    
    // println!("Time queu glyph brush section 1"); // DEBUG
    glyph_brush.queue(Section {
      text: &text,
      scale,
      screen_position: (0.0, 0.0),
      bounds: (width / 3.15, height),
      color: [0.9, 0.3, 0.3, 1.0],
      ..Section::default()
    });

    // println!("Time queu glyph brush section 2"); // DEBUG
    glyph_brush.queue(Section {
      text: &text,
      scale,
      screen_position: (width / 2.0, height / 2.0),
      bounds: (width / 3.15, height),
      color: [0.3, 0.9, 0.3, 1.0],
      layout: Layout::default()
        .h_align(HorizontalAlign::Center)
        .v_align(VerticalAlign::Center),
      ..Section::default()
    });

    // println!("Time queu glyph brush section 3"); // DEBUG
    glyph_brush.queue(Section {
      text: &text,
      scale,
      screen_position: (width, height),
      bounds: (width / 3.15, height),
      color: [0.3, 0.3, 0.9, 1.0],
      layout: Layout::default()
        .h_align(HorizontalAlign::Right)
        .v_align(VerticalAlign::Bottom),
      ..Section::default()
    });

    // println!("Time to loop over brush actions"); // DEBUG
    let mut brush_action;
    loop {
      unsafe { gl::BindTexture(gl::TEXTURE_2D, glyph_texture); }
      brush_action = glyph_brush.process_queued(
        (width as _, height as _),
        |rect, tex_data| unsafe {
          // Update part of gpu texture with new glyph alpha values
          gl::TexSubImage2D(
            gl::TEXTURE_2D,
            0,
            rect.min.x as _,
            rect.min.y as _,
            rect.width() as _,
            rect.height() as _,
            gl::RED,
            gl::UNSIGNED_BYTE,
            tex_data.as_ptr() as _,
          );
          gl_assert_ok!();
        },
        to_vertex,
      );

      // println!("Time to match brush actions for resize"); // DEBUG
      match brush_action {
        Ok(_) => break,
        Err(BrushError::TextureTooSmall { suggested, .. }) => unsafe {
          let (new_width, new_height) = suggested;
          eprint!("\r                            \r");
          eprintln!("Resizing glyph texture -> {}x{}", new_width, new_height);
          // Recreate texture as a larger size to fit more
          gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as _,
            new_width as _,
            new_height as _,
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            ptr::null(),
          );
          gl_assert_ok!();
          glyph_brush.resize_texture(new_width, new_height);
        },
      }
    }
    // println!("Time to match brush actions for draw"); // DEBUG
    match brush_action? {
      BrushAction::Draw(vertices) => {
        // Draw new vertices
        vertex_count = vertices.len();
        unsafe {
          if vertex_max < vertex_count {
            gl::BufferData(
              gl::ARRAY_BUFFER,
              (vertex_count * mem::size_of::<VertexForGlyph>()) as GLsizeiptr,
              vertices.as_ptr() as _,
              gl::DYNAMIC_DRAW,
            );
          } else {
            gl::BufferSubData(
              gl::ARRAY_BUFFER,
              0,
              (vertex_count * mem::size_of::<VertexForGlyph>()) as GLsizeiptr,
              vertices.as_ptr() as _,
            );
          }
        }
        vertex_max = vertex_max.max(vertex_count);
      }
      BrushAction::ReDraw => {}
    }

    unsafe {
      // pass 1
      gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
      gl::ClearColor(0.02, 0.02, 0.02, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);
      gl::UseProgram(program);
      gl::BindTexture(gl::TEXTURE_2D, glyph_texture);
      gl::BindVertexArray(vao);
      gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, vertex_count as _);

      // pass 2
      gl::BindFramebuffer(gl::FRAMEBUFFER, 0); // back to default framebuffer
      gl::ClearColor(0.1, 0.0, 0.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);
      gl::BindTexture(gl::TEXTURE_2D, frame_texture);
      gl::UseProgram(frame_program);
      gl::BindVertexArray(frame_vao);
      gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }

    window.swap_buffers()?;

    if let Some(rate) = loop_helper.report_rate() {
      window.set_title(&format!("{} {:.0} FPS", title, rate));
    }
    loop_helper.loop_sleep();
  }

  unsafe {
      gl::DeleteProgram(program);
      gl::DeleteShader(fs);
      gl::DeleteShader(vs);
      gl::DeleteBuffers(1, &vbo);
      gl::DeleteVertexArrays(1, &vao);
      gl::DeleteTextures(1, &glyph_texture);
      
      if using_frame_buffer {
        gl::DeleteProgram(frame_program);
        gl::DeleteShader(frame_vs);
        gl::DeleteShader(frame_fs);
        gl::DeleteBuffers(1, &frame_vbo);
        gl::DeleteVertexArrays(1, &frame_vao);
        gl::DeleteTextures(1, &frame_texture);
        gl::DeleteFramebuffers(1, &fbo);
        gl::DeleteRenderbuffers(1, &rbo);
      }
  }
  Ok(())
}

// Rendering to a texture https://learnopengl.com/Advanced-OpenGL/Framebuffers

unsafe fn make_framebuffer() -> GLuint {
  let mut fbo: GLuint = 0;
  gl::GenFramebuffers(1, &mut fbo);
  fbo
}

unsafe fn make_frame_texture(fbo: GLuint, width: GLsizei, height: GLsizei) -> GLuint {
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

unsafe fn attach_texture_to_framebuffer(fbo: GLuint, tbo: GLuint) {
  gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
  gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tbo, 0); // attach texture 
  gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
}

// framebuffer must be bound for this one
unsafe fn make_render_buffer(fbo: GLuint, width: GLsizei, height: GLsizei) -> GLuint {
  gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
  let mut rbo: GLuint = 0;
  gl::GenRenderbuffers(1, &mut rbo);
  gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
  gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width, height);
  gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
  gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
  rbo
}

unsafe fn complete_framebuffer(framebuffer: GLuint, rbo: GLuint) {
  gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
  gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo);
  if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
    panic!("framebuffer error");
  }
  gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
}

unsafe fn make_frame_quad(frame_program: GLuint) -> (GLuint, GLuint) {
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
  let vertex_size: GLsizei = mem::size_of::<FrameVertex>() as GLsizei;
  setup_attribs(frame_program, vertex_size, false, &[("aPos", 2), ("aTexCoords", 2)]).unwrap(); // ARRAY_BUFFER must also be bound for this one
  // unbind buffers
  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  gl::BindVertexArray(0);
  (vbo, vao)
}

unsafe fn setup_attribs(frame_program: GLuint, vertex_byte_size: GLsizei, useDivisor: bool, attribs: &[(&str, i32)]) -> Res<()> {
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
    if useDivisor { gl::VertexAttribDivisor(attr as _, 1); }
    offset += float_count * 4;
  }
  Ok(())
}