use std::io::{self, Write};

use url::Url;

use crate::{cli::Format, config::Config, fmt, web};

pub struct Printer {
    config: Config,
}

impl Printer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    pub async fn print(&self, url: &Url) -> Result<(), crate::Error> {
        self.link(url).await
    }
    async fn link(&self, url: &Url) -> Result<(), crate::Error> {
        let title = web::title(url).await?;

        // Truncate the title if it exceeds the max_length
        let title = match self.config.max_length {
            Some(len) => match &self.config.ellipsis {
                Some(ellipsis) => format!("{}{}", &title[..len], { ellipsis }),
                None => format!("{}...", &title[..len]),
            },
            None => title.to_string(),
        };

        let link = match self.config.format {
            Format::Markdown => fmt::markdown(url, &title),
            Format::Org => fmt::org(url, &title),
        };
        // `writeln` doesn't work well with `stdin().lock()`
        println!("{}", &link);
        Ok(())
    }
}

pub fn stdout(input: &str) {
    writeln!(io::stdout(), "{}", input).ok();
}

pub fn stderr(input: &str) {
    writeln!(io::stderr(), "{}", input).ok();
}
