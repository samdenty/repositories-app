use glob::{MatchOptions, Pattern};
use once_cell::sync::Lazy;
use reqwest::Url;

use crate::patterns;

static BADGE_PATTERNS: Lazy<&[Pattern]> = Lazy::new(|| {
  Box::leak(Box::new(patterns![
    "badges.greenkeeper.io/**",
    "img.shields.io/**",
    "travis-ci.org/**",
    "ci.appveyor.com/**",
    "api.codeclimate.com/**",
    "snyk.io/**",
    "badges.gitter.im/**",
    "codecov.io/**",
    "badge.runkitcdn.com/**",
    // with paths
    "github.com/*/*/workflows/**",
    "isitmaintained.com/badge/**",
    "d2ss6ovg47m0r5.cloudfront.net/badges/**",
    "liberapay.com/assets/widgets/**",
    "www.herokucdn.com/deploy/button.png",
    "codesandbox.io/static/img/play-codesandbox.svg",
  ]))
});

pub fn is_badge(url: &Url) -> Option<bool> {
  let url = format!("{}{}", url.domain()?, url.path());

  Some(BADGE_PATTERNS.iter().any(|url_pattern| {
    url_pattern.matches_with(
      &url,
      MatchOptions {
        case_sensitive: false,
        require_literal_separator: true,
        require_literal_leading_dot: false,
      },
    )
  }))
}
