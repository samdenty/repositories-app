use headless_chrome::protocol::browser::Bounds;
use headless_chrome::protocol::target::methods::CreateTarget;
use headless_chrome::protocol::Method;
use headless_chrome::Tab;
use headless_chrome::{protocol::page::ScreenshotFormat, Browser};
use serde::Serialize;
use std::io::Error;
use std::sync::Arc;

pub struct Renderer {
  browser: Browser,
}

impl Renderer {
  pub fn new() -> Result<Self, Error> {
    Ok(Renderer {
      browser: Browser::default().expect("failed to create browser"),
    })
  }

  pub fn load(&self, url: &str) -> Result<IconRenderer, Error> {
    let tab = self
      .browser
      .new_tab_with_options(CreateTarget {
        url,
        width: None,
        height: None,
        browser_context_id: None,
        enable_begin_frame_control: None,
      })
      .expect("failed to load iconRenderer");

    tab
      .call_method(SetDefaultBackgroundColorOverride {
        color: Color {
          r: 0,
          g: 0,
          b: 0,
          a: 0,
        },
      })
      .expect("failed to communicate with browser");

    Ok(IconRenderer::new(tab))
  }
}

pub struct IconRenderer {
  tab: Arc<Tab>,
}

impl IconRenderer {
  pub fn new(tab: Arc<Tab>) -> Self {
    IconRenderer { tab }
  }

  pub fn render(&self, resolution: u32) -> Result<Vec<u8>, Error> {
    self
      .tab
      .set_bounds(Bounds::Normal {
        height: Some(resolution / 2),
        width: Some(resolution / 2),
        left: None,
        top: None,
      })
      .expect("failed sizing window");

    self
      .tab
      .wait_for_element("body")
      .expect("failed waiting for page to load");

    let data = self
      .tab
      .capture_screenshot(ScreenshotFormat::PNG, None, true)
      .expect("failed capturing screenshot");

    Ok(data)
  }
}

#[derive(Serialize, Debug)]
struct Color {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

#[derive(Serialize, Debug)]
struct SetDefaultBackgroundColorOverride {
  color: Color,
}

impl Method for SetDefaultBackgroundColorOverride {
  const NAME: &'static str = "Emulation.setDefaultBackgroundColorOverride";
  type ReturnObject = serde_json::Value;
}

// fn screenshot(url: &str) -> Result<Vec<u8>, failure::Error> {
//   let browser = Browser::default()?;

//   tab.wait_for_element("body")?;

//   std::fs::write("test.png", &png_data);

//   Ok(png_data)
// }
