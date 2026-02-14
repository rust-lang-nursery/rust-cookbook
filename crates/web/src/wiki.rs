use regex::Regex;
use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::LazyLock;

pub fn extract_links(content: &str) -> HashSet<Cow<'_, str>> {
  static WIKI_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(
      r"(?x)
                \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
                |
                (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
            "
    )
    .unwrap()
  );

  let links: HashSet<_> = WIKI_REGEX
    .captures_iter(content)
    .map(|c| match (c.name("internal"), c.name("external")) {
        (Some(val), None) => Cow::from(val.as_str()),
        (None, Some(val)) => Cow::from(val.as_str()),
        _ => unreachable!(),
    })
    .collect::<HashSet<_>>();

  links
}
