mod renderer;
mod rsrc;
use icns::{IconFamily, Image};
use renderer::Renderer;
use std::io::{Cursor, Error};

pub struct IconManager {
  renderer: Renderer,
}

impl IconManager {
  pub fn new() -> Result<Self, Error> {
    Ok(IconManager {
      renderer: Renderer::new()?,
    })
  }

  pub fn load(&self, url: &str) -> Result<Icon, Error> {
    let icon_renderer = self.renderer.load(url)?;

    let mut icon_family = IconFamily::new();
    for &resolution in &vec![1024, 512, 256, 128, 64, 48, 32, 16] {
      let png = icon_renderer.render(resolution)?;
      let image = Image::read_png(Cursor::new(png)).unwrap();
      icon_family.add_icon(&image).unwrap();
    }

    let mut icns = Vec::new();
    icon_family.write(&mut icns).unwrap();
    let rsrc = rsrc::encode(&icns)?;

    Ok(Icon { icns, rsrc })
  }
}

pub struct Icon {
  pub icns: Vec<u8>,
  pub rsrc: Vec<u8>,
}
