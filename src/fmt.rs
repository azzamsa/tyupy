use url::Url;

/// Generates a Markdown link with the provided URL and title.
///
/// # Examples
///
/// ```
/// # use tyupy::fmt::markdown;
/// # use url::Url;
///
/// let url = Url::parse("https://example.com").unwrap();
/// let title = "Example Website";
/// let link = markdown(&url, title);
/// assert_eq!(link, "[Example Website](https://example.com/)");
/// ```
pub fn markdown(url: &Url, title: &str) -> String {
    format!("[{}]({})", title, url)
}

/// Generates an Org Mode link with the provided URL and description.
///
/// # Examples
///
/// ```
/// # use tyupy::fmt::org;
/// # use url::Url;
///
/// let url = Url::parse("https://example.com").unwrap();
/// let description = "Example Website";
/// let link = org(&url, description);
/// assert_eq!(link, "[[https://example.com/][Example Website]]");
/// ```
pub fn org(url: &Url, description: &str) -> String {
    format!("[[{}][{}]]", url, description)
}
