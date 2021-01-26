mod repo_icons;

pub use repo_icons::*;

use once_cell::sync::Lazy;
use reqwest::{header::*, Client};
use std::sync::Arc;

pub static mut CLIENT: Lazy<Arc<Client>> = Lazy::new(|| {
  let mut headers = HeaderMap::new();
  headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.104 Safari/537.36").unwrap());
  let client = Client::builder().default_headers(headers).build().unwrap();
  Arc::new(client)
});

static mut TOKEN: Option<String> = None;

pub fn get_token() -> Option<&'static String> {
  unsafe { TOKEN.as_ref() }
}

pub fn set_token<T: ToString>(token: T) {
  unsafe { TOKEN = Some(token.to_string()) };
}

#[macro_export]
macro_rules! github_api_get {
  ($($arg:tt)*) => {
    unsafe {
      use reqwest::header::AUTHORIZATION;

      let req = $crate::github::CLIENT.get(&format!("https://api.github.com/{}", format!($($arg)*)));

      if let Some(token) = $crate::github::get_token() {
        req.header(AUTHORIZATION, &format!("Bearer {}", token))
      } else {
        req
      }
    }
  }
}
