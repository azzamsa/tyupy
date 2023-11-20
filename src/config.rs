use crate::cli::Format;

#[derive(Debug, Clone)]
pub struct Config {
    /// Format the docs or check only
    pub format: Format,

    /// Max length of the title.
    pub max_length: Option<usize>,

    /// String to mark the end of truncated titles.
    pub ellipsis: Option<String>,
}
