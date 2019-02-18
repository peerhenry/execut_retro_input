use std::fs::File;
use escposify::img::Image;
use crate::spaceship_settings::*;

pub struct PlayerSettings {
  pub nickname: String,
  pub setting_values: [SpaceshipSettingValue; 4]
}

pub struct Printer {
  logo: Image,
  space_invader: Image,
  printer: escposify::printer::Printer<escposify::device::File::<File>>,
}

impl Printer {
  pub fn new(printer_address: &str) -> Printer { // "\\\\ADOLFO\\BONNETJES"
    let printer_file = escposify::device::File::<File>::new(printer_address);
    Printer {
      logo: Image::new("assets/infi.bmp"),
      space_invader: Image::new("assets/space-invader.bmp"),
      printer: escposify::printer::Printer::new(printer_file, None, None),
    }
  }

  pub fn print(&mut self, player_settings: PlayerSettings) {
    self.printer
    .font("C")
    .align("lt") // LT, CT, RT
    .style("bu")
    .size(0, 0)
    .bit_image(&self.logo, None)
    .line_space(-1)
    .text("----------------------------------")
    .text("|          SPACE INVADERS        |")
    .text("----------------------------------")
    .text("")
    .text(&format!("Nickname: {}", player_settings.nickname))
    .text("")
    .text("Stats:")
    /*.text(format!("* Rate of fire: {} bullets per second")) // todo: convert points to values
    .text(format!("* Shield strength: {} hits"))
    .text(format!("* Defense thickness: {} pixels"))
    .text(format!("* Dodging chance: {}%", ))*/
    .text(&format!("* Rate of fire: {}", player_settings.setting_values[0].value.to_string()))
    .text(&format!("* Shield strength: {}", player_settings.setting_values[1].value.to_string()))
    .text(&format!("* Defense thickness: {}", player_settings.setting_values[2].value.to_string()))
    .text(&format!("* Dodging chance: {}", player_settings.setting_values[3].value.to_string()))
    .text("")
    .text("Good luck!")
    .bit_image(&self.space_invader, None)
    .line_space(-1)
    .feed(2)
    .cut(false)
    .flush()
    .unwrap();
  }
}
