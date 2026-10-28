//! Link-definition parsing shared by the preprocessor.

/// `[label]: url`
pub fn parse_link_definition(line: &str) -> Option<(String, String)> {
    let rest = line.strip_prefix('[')?;
    let (label, url) = rest.split_once("]:")?;
    let url = url.trim();
    (!url.is_empty() && !label.contains(['[', ']'])).then(|| (label.to_string(), url.to_string()))
}
