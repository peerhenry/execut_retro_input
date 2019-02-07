pub trait Scene {
  fn new(vs_glsl: &str, fs_glsl: &str) -> Self;
  fn init(&mut self);
  fn update(&self);
  fn draw(&self);
  fn cleanup(&self);
}