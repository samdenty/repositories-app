use headless_chrome::protocol::browser::Bounds;
use headless_chrome::protocol::target::methods::CreateTarget;
use headless_chrome::protocol::Method;
use headless_chrome::Tab;
use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::time::Duration;
use std::{error::Error, sync::Mutex};
use std::{
    sync::{Arc, RwLock},
    time::Instant,
};

const BROWSER: Lazy<&'static Browser> = Lazy::new(|| {
    let launch_options = LaunchOptionsBuilder::default()
        .idle_browser_timeout(Duration::MAX)
        // .user_data_dir(Some(
        //     "/Users/samdenty/Library/Application Support/Google/Chrome".into(),
        // ))
        .build()
        .unwrap();
    let browser = Browser::new(launch_options).unwrap();
    Box::leak(Box::new(browser))
});

pub struct Renderer {
    // browser: RwLock<Browser>,
}

impl Renderer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // let launch_options = LaunchOptionsBuilder::default()
        //     .idle_browser_timeout(Duration::MAX)
        //     .build()
        //     .expect("");
        // let browser = RwLock::new(Browser::new(launch_options).expect("failed to create browser"));

        Ok(Renderer { /* browser */ })
    }

    pub fn load(&self, url: &str) -> Result<IconRenderer, Box<dyn Error>> {
        // let browser = self.browser.read()?;
        // let launch_options = LaunchOptionsBuilder::default()
        //     // .idle_browser_timeout(Duration::MAX)
        //     .build()?;
        // let browser = Browser::new(launch_options)?;

        // let browser = Browser::default()?;

        let now = Instant::now();
        let tab = BROWSER.new_tab_with_options(CreateTarget {
            url,
            width: None,
            height: None,
            browser_context_id: None,
            enable_begin_frame_control: None,
        })?;

        tab.call_method(SetDefaultBackgroundColorOverride {
            color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        })?;

        tab.evaluate(r"new Promise(resolve => (window.onload = resolve))", true)?;

        println!("created tab {:.2?}", now.elapsed());

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

    pub fn render(&self, resolution: u32) -> Result<Vec<u8>, Box<dyn Error>> {
        self.tab.set_bounds(Bounds::Normal {
            height: Some(resolution / 2),
            width: Some(resolution / 2),
            left: None,
            top: None,
        })?;

        // self
        //   .tab
        //   .wait_for_element("body")
        //   .expect("failed waiting for page to load");

        let data = self
            .tab
            .capture_screenshot(ScreenshotFormat::PNG, None, true)?;

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
