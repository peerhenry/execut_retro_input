pub struct DumbScene {
  program: GLuint;
}

impl DumbScene {
  pub new() {
    let program = setup_program();
    let vao: make_dumb_quad(phandle);
    DumbScene {
      program,
      vao 
    }
  }

  fn setup_program() -> Gluint {
    let dumb_vs = compile_shader(include_str!("shader/dumb.vert.glsl"), gl::VERTEX_SHADER)?;
    let dumb_fs = compile_shader(include_str!("shader/dumb.frag.glsl"), gl::FRAGMENT_SHADER)?;
    return link_program(dumb_vs, dumb_fs).unrwap();
  }

  pub unsafe fn render() {
    gl::UseProgram(dumb_program);
    gl::BindVertexArray(dumb_vao);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);
  }

  pub unsafe fn make_dumb_quad(dumb_program: GLuint) -> (GLuint, GLuint) {
    let vertices: Vec<GLfloat> = vec![
      // X    Y     U   V
      -1.0,  0.8,
      1.0,  1.0,
      -1.0, -1.0,
      -1.0, -1.0,
      1.0,  1.0,
      1.0, -1.0
    ];
    let buffer_byte_size = (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::BindVertexArray(vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
                  gl::ARRAY_BUFFER, // target ARRAY_BUFFER for vertex attributes
                  buffer_byte_size, // size in bytes of
                  vertices.as_ptr() as _, // pointer from which data will be copied
                  gl::STATIC_DRAW,  // expected usage of data store
                );
    let vertex_byte_size: GLsizei = (2*4) as GLsizei; // two floats
    gl::VertexAttribPointer(
      0,                // location
      2,                // size
      gl::FLOAT,        // type
      gl::FALSE as _,   // normalized
      vertex_byte_size, // stride
      0 as _            // offset
    );
    gl::EnableVertexAttribArray(0);
    // unbind buffers
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);
    vao
  }

}