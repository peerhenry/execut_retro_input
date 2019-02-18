use std::{ io::{self, Write} };
use glutin::{EventsLoop, GlWindow};
use crate::text_scene::*;
use super::Res;

pub fn handle_events(events: &mut EventsLoop, running: &mut bool, window: &GlWindow, text_scene: &mut TextScene) -> Res<()> {
  events.poll_events(|event| {
    use glutin::*;
    if let Event::WindowEvent { event, .. } = event {
      match event {
        WindowEvent::CloseRequested => *running = false,
        WindowEvent::Resized(size) => {
          let dpi = window.get_hidpi_factor();
          window.resize(size.to_physical(dpi));
          if let Some(ls) = window.get_inner_size() {
            let dimensions = ls.to_physical(dpi);
            text_scene.dimensions = dimensions;
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
          VirtualKeyCode::Escape => *running = false,
          VirtualKeyCode::Back => {
            // text_scene.pop();
          },

          // handle player 1 input (player index 0)
          VirtualKeyCode::Up => {
            text_scene.up(0);
          },
          VirtualKeyCode::Down => {
            text_scene.down(0);
          },
          VirtualKeyCode::Right => {
            text_scene.increase(0);
          },
          VirtualKeyCode::Left => {
            text_scene.decrease(0);
          },
          VirtualKeyCode::P => { // row 1, left button
            text_scene.increase(0);
          },
          VirtualKeyCode::Return => { // row 1, right button (Return is the enter key)
            text_scene.increase(0);
          },
          VirtualKeyCode::LShift => { // row 2, left button => is it left shift?
            text_scene.increase(0);
          },
          VirtualKeyCode::RShift => { // row 2, left button => or right shift?
            text_scene.increase(0);
          },
          VirtualKeyCode::Z => { // row 2, middle button
            text_scene.increase(0);
          },
          VirtualKeyCode::X => { // row 2, right button
            text_scene.increase(0);
          },
          VirtualKeyCode::LControl => { // row 3, left button
            text_scene.increase(0);
          },
          VirtualKeyCode::LAlt => { // row 3, middle button
            text_scene.increase(0);
          },
          VirtualKeyCode::Space => { // row 3, right button
            text_scene.increase(0);
          },
          VirtualKeyCode::Key5 => { // row 4, has only one button
            text_scene.increase(0);
          },

          // handle player 2 input (player index 1)
          VirtualKeyCode::R => { // p2 up
            text_scene.up(1);
          },
          VirtualKeyCode::F => { // p2 down
            text_scene.down(1);
          },
          VirtualKeyCode::G => { // p2 right
            text_scene.increase(1);
          },
          VirtualKeyCode::D => { // p2 left
            text_scene.decrease(1);
          },
          VirtualKeyCode::Key1 => { // row 1, left button
            text_scene.increase(1);
          },
          VirtualKeyCode::Key2 => { // row 1, right button
            text_scene.increase(1);
          },
          VirtualKeyCode::I => { // row 2, left button
            text_scene.increase(1);
          },
          VirtualKeyCode::J => { // row 2, middle button
            text_scene.increase(1);
          },
          VirtualKeyCode::K => { // row 2, right button
            text_scene.increase(1);
          },
          VirtualKeyCode::S => { // row 3, left button
            text_scene.increase(1);
          },
          VirtualKeyCode::Q => { // row 3, middle button
            text_scene.increase(1);
          },
          VirtualKeyCode::W => { // row 3, right button
            text_scene.increase(1);
          },
          VirtualKeyCode::A => { // row 4, just one button
            text_scene.increase(1);
          },

          _ => (),
        },

        WindowEvent::ReceivedCharacter(c) => { // obsolete
          if c != '\u{7f}' && c != '\u{8}' {
            // text_scene.push(c);
          }
        }
        WindowEvent::MouseWheel {
          delta: MouseScrollDelta::LineDelta(_, y),
          ..
        } => {
          // increase/decrease font size
          let old_size = text_scene.font_size;
          let mut size = text_scene.font_size;
          if y > 0.0 {
            size += (size / 4.0).max(2.0)
          } else {
              size *= 4.0 / 5.0
          };
          let new_size = size.max(1.0).min(2000.0);
          text_scene.font_size = new_size;
          if (new_size - old_size).abs() > 1e-2 {
            eprint!("\r                            \r");
            eprint!("font-size -> {:.1}", new_size);
            let _ = io::stderr().flush();
          }
        }
        _ => {}
      }
    }
  });
  Ok(())
}