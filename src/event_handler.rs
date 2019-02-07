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
            text_scene.pop();
          }
          _ => (),
        },
        WindowEvent::ReceivedCharacter(c) => {
          if c != '\u{7f}' && c != '\u{8}' {
            text_scene.push(c);
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