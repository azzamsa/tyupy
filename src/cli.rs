use clap::{Parser, ValueEnum};
use miette::IntoDiagnostic;
use url::Url;

#[derive(Parser)]
#[command(
    name = "tyupy",
    version,
    about = "tyupy 🐿️. \nTransform URLs into any format",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/azzamsa/tyupy/issues"
)]
pub struct Opts {
    /// The URL to be formatted
    #[arg(value_parser = parse_url)]
    pub url: Option<Url>,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = Format::Markdown)]
    pub format: Format,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    /// Markdown format
    #[value(alias = "m")]
    Markdown,
    /// Org format
    #[value(alias = "o")]
    Org,
}

pub fn parse_url(arg: &str) -> miette::Result<Url> {
    Url::parse(arg).into_diagnostic()
}
