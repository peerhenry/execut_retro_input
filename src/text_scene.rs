use glutin::dpi::PhysicalSize;
use std::{
    ffi::CString,
    mem, ptr, 
};
use gl::types::*;
use glyph_brush::{rusttype::*, *};
use crate::scene::*;
use crate::shader_compiler::*;
use crate::gl_buffers::*;
use crate::gl_error_handler::*;
use crate::helpers_for_glyph::*;
use crate::frame_buffer::*;
use crate::RETRO_COLOR_LEFT;
use crate::RETRO_COLOR_RIGHT;

#[derive(Clone, Copy)]
enum SpaceshipSetting {
  Shields,
  Firepower,
  DefenseThickness,
  DodgeChance
}

#[derive(Clone, Copy)]
struct SpaceshipSettingValue {
  setting: SpaceshipSetting,
  value: u32
}

impl SpaceshipSettingValue {
  fn new(setting: SpaceshipSetting) -> Self {
    SpaceshipSettingValue {
      setting,
      value: 0
    }
  }
}

enum SelectedInput {
  Setting(SpaceshipSetting),
  Submit
}

pub struct TextScene<'a> {
  program: ShaderProgram,
  vbo: GLuint,
  vao: GLuint,
  glyph_texture: GLuint,
  glyph_brush: GlyphBrush<'a>,
  text: String,
  pub font_size: f32,
  vertex_count: usize,
  vertex_max: usize,
  pub dimensions: PhysicalSize,
  frame_buffer: Option<Framebuffer>,
  font_tex_loc: GLint,
  selected_input: SelectedInput,
  points_remaining: u32,
  setting_points: [SpaceshipSettingValue; 4]
}

impl TextScene<'_> {
  pub fn new(vs_glsl: &str, fs_glsl: &str, window: &glutin::GlWindow, frame_buffer: Option<Framebuffer>) -> Self {
    let font_bytes: &[u8] = include_bytes!("../fonts/retro computer_demo.ttf");
    let glyph_brush: GlyphBrush = GlyphBrushBuilder::using_font_bytes(font_bytes).build();
    // let text: String = include_str!("text/lipsum.txt").into();
    let text: String = include_str!("text/input.txt").into();
    let dimensions = window
      .get_inner_size()
      .ok_or("get_inner_size = None").unwrap()
      .to_physical(window.get_hidpi_factor());
    let program = build_shader_program(vs_glsl, fs_glsl).unwrap();
    let font_tex_loc: GLint;
    unsafe { font_tex_loc = gl::GetUniformLocation(program.handle, CString::new("baseRand").unwrap().as_ptr()); }
    let settings: [SpaceshipSettingValue; 4] = [
      SpaceshipSettingValue::new(SpaceshipSetting::Shields),
      SpaceshipSettingValue::new(SpaceshipSetting::Firepower),
      SpaceshipSettingValue::new(SpaceshipSetting::DefenseThickness),
      SpaceshipSettingValue::new(SpaceshipSetting::DodgeChance)
    ];
    TextScene {
      program,
      vbo: 0,
      vao: 0,
      glyph_texture: 0,
      glyph_brush,
      text,
      font_size: 36.0, // was 18.0 from example
      vertex_count: 0,
      vertex_max: 0,
      dimensions,
      frame_buffer,
      font_tex_loc,
      selected_input: SelectedInput::Setting(SpaceshipSetting::Shields),
      points_remaining: 10,
      setting_points: settings
    }
  }

  // to become obsolete
  pub fn pop(&mut self) {
    self.text.pop();
  }

  // to become obsolete
  pub fn push(&mut self, c: char) {
    if c != '\u{7f}' && c != '\u{8}' {
      self.text.push(c);
    }
  }

  pub fn up(&mut self) {
    let new_selected: SelectedInput;
    match self.selected_input {
      SelectedInput::Setting(setting) => {
        match setting {
          SpaceshipSetting::Shields => { new_selected = SelectedInput::Submit; },
          SpaceshipSetting::Firepower => { new_selected = SelectedInput::Setting(SpaceshipSetting::Shields); },
          SpaceshipSetting::DefenseThickness => { new_selected = SelectedInput::Setting(SpaceshipSetting::Firepower); },
          SpaceshipSetting::DodgeChance => { new_selected = SelectedInput::Setting(SpaceshipSetting::DefenseThickness); },
        }
      },
      SelectedInput::Submit => {
        new_selected = SelectedInput::Setting(SpaceshipSetting::DodgeChance);
      }
    }
    self.selected_input = new_selected;
  }

  pub fn down(&mut self) {
    let new_selected: SelectedInput;
    match self.selected_input {
      SelectedInput::Setting(setting) => {
        match setting {
          SpaceshipSetting::Shields => { new_selected = SelectedInput::Setting(SpaceshipSetting::Firepower); },
          SpaceshipSetting::Firepower => { new_selected = SelectedInput::Setting(SpaceshipSetting::DefenseThickness); },
          SpaceshipSetting::DefenseThickness => { new_selected = SelectedInput::Setting(SpaceshipSetting::DodgeChance); },
          SpaceshipSetting::DodgeChance => { new_selected = SelectedInput::Submit; },
        }
      },
      SelectedInput::Submit => {
        new_selected = SelectedInput::Setting(SpaceshipSetting::Shields);
      }
    }
    self.selected_input = new_selected;
  }

  fn change_setting(&mut self, index: usize, delta: i32)
  {
    let new_val: i32 = self.setting_points[index].value as i32 + delta;
    let new_remaining = self.points_remaining as i32- delta;
    if new_remaining >= 0 && new_remaining <= 10 && new_val >= 0 {
      self.setting_points[index].value = new_val as u32;
      self.points_remaining = new_remaining as u32;
    }
  }

  fn change(&mut self, delta: i32) {
    match self.selected_input {
      SelectedInput::Setting(setting) => {
        match setting {
          SpaceshipSetting::Shields => { self.change_setting(0, delta); },
          SpaceshipSetting::Firepower => { self.change_setting(1, delta); },
          SpaceshipSetting::DefenseThickness => { self.change_setting(2, delta); },
          SpaceshipSetting::DodgeChance => { self.change_setting(3, delta); },
        }
      },
      SelectedInput::Submit => {
        self.selected_input = SelectedInput::Setting(SpaceshipSetting::Shields);
        // todo:
        self.points_remaining = 10;
        self.setting_points[0].value = 0;
        self.setting_points[1].value = 0;
        self.setting_points[2].value = 0;
        self.setting_points[3].value = 0;
        println!("TODO: SAVE SETTINGS");
      }
    }
  }

  pub fn increase(&mut self) {
    self.change(1);
  }

  pub fn decrease(&mut self) {
    self.change(-1);
  }

  fn generate_string(&mut self) -> String {
    let mut lines: Vec<String> = vec![
      String::from("Welcome honorable guest."),
      String::from(""),
      String::from("Prepare for space invaders!"),
      String::from("Please input your spaceship settings..."),
      String::from(""),
      String::from(format!("Points remaining: {}", self.points_remaining)),
      String::from(""),
    ];
    for (i, elem) in self.setting_points.iter_mut().enumerate() {
      let settingName: &str;
      let points: u32 = elem.value;
      match elem.setting {
        SpaceshipSetting::Shields => { settingName = "  Shields"; },
        SpaceshipSetting::Firepower => { settingName = "  Firepower"; },
        SpaceshipSetting::DefenseThickness => { settingName = "  DefenseThickness"; },
        SpaceshipSetting::DodgeChance => { settingName = "  DodgeChance"; },
      }
      let new_line: String = format!("{}: {}", settingName, points);
      lines.push(new_line);
    }
    lines.push(String::from(" "));
    lines.push(String::from("  SUBMIT"));
    match self.selected_input {
      SelectedInput::Setting(setting) => {
        match setting {
          SpaceshipSetting::Shields => { lines[7] = lines[7].replace("  ", "> "); },
          SpaceshipSetting::Firepower => { lines[8] = lines[8].replace("  ", "> "); },
          SpaceshipSetting::DefenseThickness => { lines[9] = lines[9].replace("  ", "> "); },
          SpaceshipSetting::DodgeChance => { lines[10] = lines[10].replace("  ", "> "); },
        }
      },
      SelectedInput::Submit => {
        lines[12] = lines[12].replace("  ", "> "); 
      }
    }
    lines.join("\n")
  }

  pub fn update(&mut self, window: &glutin::GlWindow) {
    // vvvv glyph brush queue vvvv
    let width = self.dimensions.width as f32; // use this if you render to viewport
    let height = self.dimensions.height as _;
    let scale = Scale::uniform((self.font_size * window.get_hidpi_factor() as f32).round());
    let input_string = self.generate_string();
    self.glyph_brush.queue(Section {
      text: &input_string,
      scale,
      screen_position: (width/20.0, height/20.0),
      bounds: (width/2.0, height),
      color: RETRO_COLOR_LEFT,
      ..Section::default()
    });

    let input_string_right = self.generate_string();
    self.glyph_brush.queue(Section {
      text: &input_string,
      scale,
      screen_position: (width - width/20.0, height/20.0),
      bounds: (width/2.0, height),
      color: RETRO_COLOR_RIGHT,
      layout: Layout::default()
        .h_align(HorizontalAlign::Right),
        //.v_align(VerticalAlign::Bottom),
      ..Section::default()
    });

    /*
    // println!("Time queu glyph brush section 1"); // DEBUG
    self.glyph_brush.queue(Section {
      text: &self.text,
      scale,
      screen_position: (0.0, 0.0),
      bounds: (width / 3.15, height),
      color: [0.9, 0.3, 0.3, 1.0],
      ..Section::default()
    });
    
    // println!("Time queu glyph brush section 2"); // DEBUG
    self.glyph_brush.queue(Section {
      text: &self.text,
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
    self.glyph_brush.queue(Section {
      text: &self.text,
      scale,
      screen_position: (width, height),
      bounds: (width / 3.15, height),
      color: [0.3, 0.3, 0.9, 1.0],
      layout: Layout::default()
        .h_align(HorizontalAlign::Right)
        .v_align(VerticalAlign::Bottom),
      ..Section::default()
    });
    // ^^^^ glyph brush queue ^^^^
    */

    // vvvv handle glyph brush action vvvv
    // println!("Time to loop over brush actions"); // DEBUG
    let mut brush_action;
    loop {
      unsafe { gl::BindTexture(gl::TEXTURE_2D, self.glyph_texture); }
      brush_action = self.glyph_brush.process_queued(
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
          self.glyph_brush.resize_texture(new_width, new_height);
        },
      }
    }
    // println!("Time to match brush actions for draw"); // DEBUG
    match brush_action.unwrap() {
      BrushAction::Draw(vertices) => {
        // Draw new vertices
        self.vertex_count = vertices.len();
        unsafe {
          if self.vertex_max < self.vertex_count {
            gl::BufferData(
              gl::ARRAY_BUFFER,
              (self.vertex_count * mem::size_of::<VertexForGlyph>()) as GLsizeiptr,
              vertices.as_ptr() as _,
              gl::DYNAMIC_DRAW,
            );
          } else {
            gl::BufferSubData(
              gl::ARRAY_BUFFER,
              0,
              (self.vertex_count * mem::size_of::<VertexForGlyph>()) as GLsizeiptr,
              vertices.as_ptr() as _,
            );
          }
        }
        self.vertex_max = self.vertex_max.max(self.vertex_count);
      }
      BrushAction::ReDraw => {}
    }
    // ^^^^ handle glyph brush action ^^^^
  }
}

// ==== impl Scene ===

impl Scene for TextScene<'_> {
  fn init(&mut self) {
    unsafe {
      let mut vao = 0;
      let mut vbo = 0;
      let mut glyph_texture = 0;
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
        let (width, height) = self.glyph_brush.texture_dimensions();
        println!("glyph_brush w, h: {}, {}", width, height);
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
        self.vao = vao;
        self.vbo = vbo;
        self.glyph_texture = glyph_texture;
        gl_assert_ok!();
      }
      
      // Use shader program
      gl::UseProgram(self.program.handle);
      gl::BindFragDataLocation(self.program.handle, 0, CString::new("out_color").unwrap().as_ptr());

      // Specify the layout of the vertex data
      setup_attribs(
        self.program.handle, 
        mem::size_of::<VertexForGlyph>() as _, 
        true,
        &[
          ("left_top", 3),
          ("right_bottom", 2),
          ("tex_left_top", 2),
          ("tex_right_bottom", 2),
          ("color", 4),
        ]
      ).unwrap();

      // Enabled alpha blending
      gl::Enable(gl::BLEND);
      gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
      // Use srgb for consistency with other examples
      gl::Enable(gl::FRAMEBUFFER_SRGB);
      gl::ClearColor(0.02, 0.02, 0.02, 1.0);
      // vao is used after this somewhere...
    }
  }

  unsafe fn draw(&self) {
    if let Some(frame_buffer) = self.frame_buffer { frame_buffer.bind(); }
    else { gl::BindFramebuffer(gl::FRAMEBUFFER, 0); }
    gl::ClearColor(0.02, 0.02, 0.02, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
    gl::UseProgram(self.program.handle);
    gl::BindTexture(gl::TEXTURE_2D, self.glyph_texture);
    gl::Uniform1i(self.font_tex_loc, 0);

    gl::BindVertexArray(self.vao);
    gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, self.vertex_count as _);
  }

  fn cleanup(&self) {
    unsafe {
      self.program.cleanup();
      gl::DeleteBuffers(1, &self.vbo);
      gl::DeleteVertexArrays(1, &self.vao);
      gl::DeleteTextures(1, &self.glyph_texture);
      if let Some(frame_buffer) = self.frame_buffer { frame_buffer.cleanup(); }
    }
  }
}