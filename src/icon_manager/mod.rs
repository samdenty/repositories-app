mod renderer;
mod rsrc;
use colored::*;
use icns::{IconFamily, Image};
use image::{imageops::FilterType, io::Reader as ImageReader, GenericImageView, ImageBuffer};
use renderer::Renderer;
use std::{
    collections::HashMap,
    convert::TryInto,
    error::Error,
    io::Cursor,
    sync::{Arc, RwLock},
};

pub struct IconManager {
    renderer: Renderer,
}

lazy_static! {
    static ref CACHE: RwLock<HashMap<String, Arc<Icon>>> = RwLock::new(HashMap::new());
}

impl IconManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(IconManager {
            renderer: Renderer::new()?,
        })
    }

    pub async fn load_user(&self, login: &str) -> Result<Arc<Icon>, Box<dyn Error>> {
        if let Some(icon) = CACHE.read()?.get(login) {
            return Ok(icon.clone());
        }

        println!("{} {}", "start rendering".green(), login);
        let now = std::time::Instant::now();
        let mut icon_family = IconFamily::new();
        let result = reqwest::get(&format!("https://github.com/{}.png?size=512", login))
            .await?
            .bytes()
            .await?;

        let image = ImageReader::new(Cursor::new(result))
            .with_guessed_format()?
            .decode()?;

        let resolution = [16, 32, 48, 64, 128, 256, 512]
            .iter()
            .find(|&&resolution| image.height() <= resolution)
            .copied()
            .unwrap_or(1024);

        let mut image = image.resize(resolution, resolution, FilterType::Nearest);

        let overlay = ImageBuffer::from_fn(resolution, resolution, |x, y| {
            if (x + y) % 2 == 0 {
                image::Rgba([0, 0, 0, 150])
            } else {
                image::Rgba([255, 255, 255, 150])
            }
        });

        image::imageops::overlay(&mut image, &overlay, 0, 0);

        icon_family.add_icon(&image.try_into()?)?;

        let mut icns = Vec::new();
        icon_family.write(&mut icns)?;
        let rsrc = rsrc::encode(&icns)?;

        println!(
            "{} {:.2?} {}",
            "done rendering".green(),
            now.elapsed(),
            rsrc.len(),
        );
        let icon = Arc::new(Icon { icns, rsrc });

        // self.cache.insert(login.to_string(), icon.clone());

        Ok(icon)
    }

    pub fn load_repo(&self, url: &str) -> Result<Arc<Icon>, Box<dyn Error>> {
        if let Some(icon) = CACHE.read()?.get(url) {
            return Ok(icon.clone());
        }

        println!("{} {}", "start rendering".green(), url);
        let now = std::time::Instant::now();

        let icon_renderer = self.renderer.load(url)?;

        let mut icon_family = IconFamily::new();
        for &resolution in &[512, 1024] {
            let time = std::time::Instant::now();
            let png = icon_renderer.render(resolution)?;
            let image = Image::new_png(png, resolution, resolution);
            icon_family.add_icon(&image)?;
            println!("render {} {:.2?}", resolution, time.elapsed());
        }

        let mut icns = Vec::new();
        icon_family.write(&mut icns)?;

        std::fs::write("./test.icns", &icns)?;
        let rsrc = rsrc::encode(&icns)?;

        let icon = Arc::new(Icon { icns, rsrc });

        CACHE.write()?.insert(url.to_string(), icon.clone());

        println!(
            "{} {:.2?} {}",
            "done rendering".green(),
            now.elapsed(),
            icon.rsrc.len()
        );

        Ok(icon)
    }
}

pub struct Icon {
    pub icns: Vec<u8>,
    pub rsrc: Vec<u8>,
}
