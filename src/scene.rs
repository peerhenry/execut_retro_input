pub trait Scene {
  fn init(&mut self);
  fn draw(&self);
  fn cleanup(&self);
}