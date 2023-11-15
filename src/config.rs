use crate::cli::Format;

#[derive(Debug, Clone)]
pub struct Config {
    /// Format the docs or check only
    pub format: Format,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            format: Format::Markdown,
        }
    }
}
