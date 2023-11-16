use std::io::{self, Write};

use crate::{cli::Format, config::Config, fmt, web};

pub struct Printer {
    config: Config,
}

impl Printer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    pub fn print(&self, url: &str) -> Result<(), crate::Error> {
        self.link(url)
    }
    fn link(&self, url: &str) -> Result<(), crate::Error> {
        let title = web::title(url)?;
        let link = match self.config.format {
            Format::Markdown => fmt::markdown(url, &title),
            Format::Org => fmt::org(url, &title),
        };
        stdout(&link);
        Ok(())
    }
}

pub fn stdout(input: &str) {
    writeln!(io::stdout(), "{}", input).ok();
}

pub fn stderr(input: &str) {
    writeln!(io::stderr(), "{}", input).ok();
}
