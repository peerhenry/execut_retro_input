use std::fs::File;

use escposify::img::Image;

pub struct Printer {
  logo: Image,
  space_invader: Image,
  printer: escposify::printer::Printer<escposify::device::File::<File>>,
}

impl Printer {
  pub fn new() -> Printer {
    let printer_file = escposify::device::File::<File>::new("\\\\ADOLFO\\BONNETJES");

    Printer {
      logo: Image::new("infi.bmp"),
      space_invader: Image::new("space-invader.bmp"),
      printer: escposify::printer::Printer::new(printer_file, None, None),
    }
  }

  pub fn print(&mut self) {
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
    .text("Nickname: Angry Darth Vader")
    .text("")
    .text("Stats:")
    .text("* Rate of fire: 5 bullets per second")
    .text("* Shield strength: 10 hits")
    .text("* Defense thickness: 10 pixels")
    .text("* Dodging chance: 20%")
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
