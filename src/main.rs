#![deny(unsafe_code)]
use std::process;

use clap::Parser;
use miette::Result;

use tyupy::{
    cli::{Format, Opts},
    config::Config,
    exit_codes::ExitCode,
    fmt, out, web,
};

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            out::stderr(&format!("Error: {:?}", err));
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn run() -> Result<ExitCode> {
    let opts = Opts::parse();

    let url = &opts.url.clone();
    let config = construct_config(opts);

    let title = web::title(url)?;
    let link = match config.format {
        Format::Markdown => fmt::markdown(url, &title),
        Format::Org => fmt::org(url, &title),
    };
    out::stdout(&link);

    Ok(ExitCode::Success)
}

fn construct_config(opts: Opts) -> Config {
    Config {
        format: opts.format,
    }
}
