use gl::types::*;
use glyph_brush::{rusttype::*};
use std::{
    ffi::CString,
    ptr, str,
};

pub type Res<T> = Result<T, Box<std::error::Error>>;
/// `[left_top * 3, right_bottom * 2, tex_left_top * 2, tex_right_bottom * 2, color * 4]`
pub type VertexForGlyph = [GLfloat; 13];

// main was here

pub fn gl_err_to_str(err: u32) -> &'static str {
  match err {
    gl::INVALID_ENUM => "INVALID_ENUM",
    gl::INVALID_VALUE => "INVALID_VALUE",
    gl::INVALID_OPERATION => "INVALID_OPERATION",
    gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
    gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
    gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
    gl::STACK_OVERFLOW => "STACK_OVERFLOW",
    _ => "Unknown error",
  }
}

pub fn compile_shader(src: &str, ty: GLenum) -> Res<GLuint> {
  let shader;
  unsafe {
    shader = gl::CreateShader(ty);
    // Attempt to compile the shader
    let c_str = CString::new(src.as_bytes())?;
    gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    // Get the compile status
    let mut status = GLint::from(gl::FALSE);
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

    // Fail on error
    if status != GLint::from(gl::TRUE) {
      let mut len = 0;
      gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
      let mut buf = Vec::with_capacity(len as usize);
      buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
      gl::GetShaderInfoLog(
          shader,
          len,
          ptr::null_mut(),
          buf.as_mut_ptr() as *mut GLchar,
      );
      return Err(str::from_utf8(&buf)?.into());
    }
  }
  Ok(shader)
}

pub fn link_program(vs: GLuint, fs: GLuint) -> Res<GLuint> {
  unsafe {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    // Get the link status
    let mut status = GLint::from(gl::FALSE);
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    // Fail on error
    if status != GLint::from(gl::TRUE) {
      let mut len: GLint = 0;
      gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
      let mut buf = Vec::with_capacity(len as usize);
      buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
      gl::GetProgramInfoLog(
        program,
        len,
        ptr::null_mut(),
        buf.as_mut_ptr() as *mut GLchar,
      );
      return Err(str::from_utf8(&buf)?.into());
    }
    Ok(program)
  }
}

#[inline]
pub fn to_vertex(
  glyph_brush::GlyphVertex {
    mut tex_coords,
    pixel_coords,
    bounds,
    screen_dimensions: (screen_w, screen_h),
    color,
    z,
  }: glyph_brush::GlyphVertex,
) -> VertexForGlyph {
  let gl_bounds = Rect {
      min: point(
          2.0 * (bounds.min.x / screen_w - 0.5),
          2.0 * (0.5 - bounds.min.y / screen_h),
      ),
      max: point(
          2.0 * (bounds.max.x / screen_w - 0.5),
          2.0 * (0.5 - bounds.max.y / screen_h),
      ),
  };

  let mut gl_rect = Rect {
    min: point(
      2.0 * (pixel_coords.min.x as f32 / screen_w - 0.5),
      2.0 * (0.5 - pixel_coords.min.y as f32 / screen_h),
    ),
    max: point(
      2.0 * (pixel_coords.max.x as f32 / screen_w - 0.5),
      2.0 * (0.5 - pixel_coords.max.y as f32 / screen_h),
    ),
  };

  // handle overlapping bounds, modify uv_rect to preserve texture aspect
  if gl_rect.max.x > gl_bounds.max.x {
    let old_width = gl_rect.width();
    gl_rect.max.x = gl_bounds.max.x;
    tex_coords.max.x = tex_coords.min.x + tex_coords.width() * gl_rect.width() / old_width;
  }
  if gl_rect.min.x < gl_bounds.min.x {
    let old_width = gl_rect.width();
    gl_rect.min.x = gl_bounds.min.x;
    tex_coords.min.x = tex_coords.max.x - tex_coords.width() * gl_rect.width() / old_width;
  }
  // note: y access is flipped gl compared with screen,
  // texture is not flipped (ie is a headache)
  if gl_rect.max.y < gl_bounds.max.y {
    let old_height = gl_rect.height();
    gl_rect.max.y = gl_bounds.max.y;
    tex_coords.max.y = tex_coords.min.y + tex_coords.height() * gl_rect.height() / old_height;
  }
  if gl_rect.min.y > gl_bounds.min.y {
    let old_height = gl_rect.height();
    gl_rect.min.y = gl_bounds.min.y;
    tex_coords.min.y = tex_coords.max.y - tex_coords.height() * gl_rect.height() / old_height;
  }

  [
    gl_rect.min.x,
    gl_rect.max.y,
    z,
    gl_rect.max.x,
    gl_rect.min.y,
    tex_coords.min.x,
    tex_coords.max.y,
    tex_coords.max.x,
    tex_coords.min.y,
    color[0],
    color[1],
    color[2],
    color[3],
  ]
}
