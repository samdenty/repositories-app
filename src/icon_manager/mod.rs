mod renderer;
mod rsrc;
use icns::{IconFamily, Image};
use renderer::Renderer;
use std::{
  collections::HashMap,
  io::{Cursor, Error},
  rc::Rc,
};

pub struct IconManager {
  renderer: Renderer,
  cache: HashMap<String, Rc<Icon>>,
}

impl IconManager {
  pub fn new() -> Result<Self, Error> {
    Ok(IconManager {
      renderer: Renderer::new()?,
      cache: HashMap::new(),
    })
  }

  pub fn load(&mut self, url: &str) -> Result<Rc<Icon>, Error> {
    if let Some(icon) = self.cache.get(url) {
      return Ok(icon.clone());
    }

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

    let icon = Rc::new(Icon { icns, rsrc });

    self.cache.insert(url.to_string(), icon.clone());

    Ok(icon)
  }
}

pub struct Icon {
  pub icns: Vec<u8>,
  pub rsrc: Vec<u8>,
}
