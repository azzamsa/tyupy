/// Generates a Markdown link with the provided URL and title.
///
/// # Examples
///
/// ```
/// use tyupy::fmt::markdown;
/// let url = "https://example.com";
/// let title = "Example Website";
/// let markdown_link = markdown(url, title);
/// assert_eq!(markdown_link, "[Example Website](https://example.com)");
/// ```
pub fn markdown(url: &str, title: &str) -> String {
    format!("[{}]({})", title, url)
}

/// Generates an Org Mode link with the provided URL and description.
///
/// # Examples
///
/// ```
/// use tyupy::fmt::org;
/// let url = "https://example.com";
/// let description = "Example Website";
/// let org_link = org(url, description);
/// assert_eq!(org_link, "[[https://example.com][Example Website]]");
/// ```
pub fn org(url: &str, description: &str) -> String {
    format!("[[{}][{}]]", url, description)
}
