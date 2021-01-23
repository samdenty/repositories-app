#[macro_export]
macro_rules! regex {
  ($re:literal $(,)?) => {{
    static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
    RE.get_or_init(|| regex::Regex::new($re).unwrap())
  }};
}

#[macro_export]
macro_rules! github_api {
  ($($arg:tt)*) => {{
      let res = format!("https://api.github.com/{}", format!($($arg)*));
      res
  }}
}
