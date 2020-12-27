mod renderer;
mod rsrc;
use colored::*;
use icns::IconFamily;
use image::{imageops::FilterType, io::Reader as ImageReader, GenericImageView};
use renderer::Renderer;
use std::convert::TryInto;
use std::error::Error;
use std::{collections::HashMap, io::Cursor, sync::Arc};

pub struct IconManager {
    // renderer: Renderer,
    cache: HashMap<String, Arc<Icon>>,
}

impl IconManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(IconManager {
            // renderer: Renderer::new()?,
            cache: HashMap::new(),
        })
    }

    // pub fn load_repo(&mut self, url: &str) -> Result<Arc<Icon>, Box<dyn Error>> {
    //     if let Some(icon) = self.cache.get(url) {
    //         return Ok(icon.clone());
    //     }

    //     println!("{} {}", "start rendering".green(), url);
    //     let now = std::time::Instant::now();

    //     let icon_renderer = self.renderer.load(url)?;

    //     let mut icon_family = IconFamily::new();
    //     for &resolution in &[/* 1024, */ 512 /* 256, 128, 64, 48, 32, 16 */] {
    //         let png = icon_renderer.render(resolution)?;
    //         let image = Image::read_png(Cursor::new(png))?;
    //         icon_family.add_icon(&image)?;
    //     }

    //     let mut icns = Vec::new();
    //     icon_family.write(&mut icns)?;
    //     let rsrc = rsrc::encode(&icns)?;

    //     let icon = Arc::new(Icon { icns, rsrc });

    //     self.cache.insert(url.to_string(), icon.clone());

    //     println!(
    //         "{} {:.2?} {}",
    //         "done rendering".green(),
    //         now.elapsed(),
    //         icon.rsrc.len()
    //     );

    //     Ok(icon)
    // }

    pub async fn load_user(&self, login: &str) -> Result<Arc<Icon>, Box<dyn Error>> {
        if let Some(icon) = self.cache.get(login) {
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

        let image = image.resize(resolution, resolution, FilterType::Nearest);

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
}

pub struct Icon {
    pub icns: Vec<u8>,
    pub rsrc: Vec<u8>,
}
