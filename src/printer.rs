use std::fs::File;

use escposify::img::Image;

pub struct Printer {
  logo: Image,
  printer: escposify::printer::Printer<escposify::device::File::<File>>,
}

impl Printer {
  pub fn new() -> Printer {
    let printer_file = escposify::device::File::<File>::new("\\\\ADOLFO\\BONNETJES");

    Printer {
      logo: Image::new("infi2.bmp"),
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
    .text("|                                |")
    .text("|          SPACE INVADERS        |")
    .text("|                                |")
    .text("----------------------------------")
    .text("Stats:")
    .text("* Armor: 50")
    .text("* Health: 20")
    .text("Test")
    .feed(2)
    .cut(false)
    .flush()
    .unwrap();
  }
}
