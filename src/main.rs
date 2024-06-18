#![deny(unsafe_code)]
use std::io;
use std::io::BufRead;
use std::process;

use clap::Parser;

use tyupy::{
    cli::{self, Opts},
    config::Config,
    exit_codes::ExitCode,
    output,
};

#[tokio::main]
async fn main() {
    let result = run();
    match result.await {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            output::stderr(&format!("Error: {:?}", err));
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

async fn run() -> miette::Result<ExitCode> {
    let opts = Opts::parse();
    let config = Config {
        format: opts.format,
        max_length: opts.max_length,
        ellipsis: opts.ellipsis,
    };

    let printer = output::Printer::new(config);
    match &opts.url {
        Some(url) => printer.print(url).await?,
        None => {
            for line in io::stdin().lock().lines() {
                let line = &line.unwrap_or("".to_string());
                let url = cli::parse_url(line)?;
                printer.print(&url).await?
            }
        }
    };

    Ok(ExitCode::Success)
}
