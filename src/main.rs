#![deny(unsafe_code)]
use std::io;
use std::io::BufRead;
use std::process;

use clap::Parser;
use miette::Result;

use tyupy::{cli::Opts, config::Config, exit_codes::ExitCode, output};

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            output::stderr(&format!("Error: {:?}", err));
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn run() -> Result<ExitCode> {
    let opts = Opts::parse();
    let config = Config {
        format: opts.format,
    };

    let printer = output::Printer::new(config);
    match opts.url.clone() {
        Some(url) => printer.print(&url)?,
        None => {
            for line in io::stdin().lock().lines() {
                let url = line.unwrap_or("".to_string());
                printer.print(&url)?
            }
        }
    };

    Ok(ExitCode::Success)
}
